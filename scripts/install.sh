#!/usr/bin/env bash
# Install EPOS GSX 300 daemon + udev rules + systemd user service + GUI.
#
# Usage:
#   ./scripts/install.sh              install daemon + service (user)
#   ./scripts/install.sh --gui        install GUI desktop app (Tauri binary + desktop entry)
#   ./scripts/install.sh --udev       install udev rule (needs sudo)
#   ./scripts/install.sh --system     system-wide install into /usr/local
#   ./scripts/install.sh --uninstall  remove everything installed by this script
#
# Option B (3i): also installs the per-role epos PipeWire instances
# (pipewire-epos@{eq,voice,sidetone}) + the static fail-closed null-sink
# anchor in MAIN (40-epos-eq-virtualsink.conf) and migrates the OLD
# daemon-generated main-pipewire DSP confs (50-epos-eq / 51-epos-voice /
# 93-epos-noisegate) out of the way.
#
# IMPORTANT (ONE-TIME, do when you are ready to restart your audio session):
#   The MAIN pipewire graph is restarted exactly once at install to load the
#   null-sink anchor. Nothing here restarts main automatically.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_NAME="epos-gsx300d"
GUI_BIN="epos-gsx300-gui"
UDEV_RULE="70-epos-gsx300.rules"
SERVICE="epos-gsx300d.service"
EPOS_PW_SERVICE="pipewire-epos@.service"
EQ_NULL_SINK_CONF="40-epos-eq-virtualsink.conf"
DESKTOP_FILE="epos-gsx300-gui.desktop"
ICON_SIZE=128

# --- paths ---------------------------------------------------------------
PREFIX="${PREFIX:-$HOME/.local}"
BIN_DIR="$PREFIX/bin"
SERVICE_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user"
APPS_DIR="$PREFIX/share/applications"
ICONS_DIR="$PREFIX/share/icons/hicolor"
UDEV_DIR="/etc/udev/rules.d"
RUST_LOG_DEFAULT="${RUST_LOG_DEFAULT:-info}"

# Option B paths
PW_CONF_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/pipewire/pipewire.conf.d"
WP_CONF_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/wireplumber/wireplumber.conf.d"
EPOS_CONF_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/pipewire-epos"
# OLD daemon-managed DSP confs from the pre-Option-B design — removing them
# prevents double-processing (old main chains + new epos instances) after the
# one-time main restart. The daemon no longer writes these files.
OLD_MAIN_DSP_CONFS=( "50-epos-eq.conf" "51-epos-voice-enhancer.conf" "93-epos-noisegate.conf" )
EPOS_INSTANCE_ROLES=( eq voice sidetone )

banner() { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
ok()     { printf '\033[1;32m  ✔\033[0m %s\n' "$*"; }
warn()   { printf '\033[1;33m  ⚠\033[0m %s\n' "$*" >&2; }
die()    { printf '\033[1;31m✖\033[0m %s\n' "$*" >&2; exit 1; }

_is_udev_user() {
    # Non-root installs are user-level; udev rules always need root.
    return 1
}

_need_build() {
    if [[ ! -x "$ROOT/target/release/$BIN_NAME" ]]; then
        banner "Building daemon (release)..."
        ( cd "$ROOT" && cargo build --release --bin "$BIN_NAME" )
    fi
}

# Migrate away the OLD daemon-managed main-pipewire DSP confs. They take
# effect only on the next MAIN restart (conf.d is read at startup), which is
# the same one-time restart that loads the null-sink anchor.
_cleanup_old_main_dsp_confs() {
    local removed=0 f
    for f in "${OLD_MAIN_DSP_CONFS[@]}"; do
        if [[ -f "$PW_CONF_DIR/$f" ]]; then
            rm -f "$PW_CONF_DIR/$f"
            warn "removed OLD main-pipewire DSP conf (migrated to epos instances): $f"
            removed=1
        fi
    done
    [[ $removed -eq 1 ]] && \
        warn "old chains remain loaded in the RUNNING main instance until its one-time restart"
}

# Install the per-role epos PipeWire instances + MAIN null-sink anchor.
# Instances are only *enabled* (start with the next graphical session), NOT
# started now: the eq instance binds epos-eq-input.monitor which does not
# exist until the one-time main restart, so starting it here would crash-loop.
_install_pw_epos() {
    mkdir -p "$SERVICE_DIR" "$PW_CONF_DIR" "$EPOS_CONF_DIR"
    mkdir -p "$WP_CONF_DIR"

    install -m 0644 "$ROOT/systemd/$EPOS_PW_SERVICE" "$SERVICE_DIR/$EPOS_PW_SERVICE"
    ok "instance template -> $SERVICE_DIR/$EPOS_PW_SERVICE"

    # The static fail-closed null-sink is no longer installed. It existed to be
    # the sink the EQ chain recorded the monitor of, and the chain is now its own
    # sink, so nothing points at it. Left in place it is worse than useless: it
    # appears in the device list as a second EPOS-looking output that plays
    # nothing, which is exactly the thing a user cannot tell from the real one.
    # Its fail-closed job is done by the watchdog instead - measured: stopping the
    # EQ instance with playback on the chain's sink moved no stream to the
    # speakers, and the chain was back in about four seconds.
    rm -f "$PW_CONF_DIR/$EQ_NULL_SINK_CONF"
    ok "removed any leftover EQ null-sink anchor from an older install"

    # No WirePlumber routing rule is installed. Output routing is owned by the
    # daemon (AudioPipeline::route_output): it moves the PipeWire default sink
    # to epos-eq-input when the EQ is enabled. A WirePlumber rule was tried and
    # measured to be either a no-op (target.object on a device node) or a
    # feedback loop (rewriting stream targets catches the EQ instance's own
    # output). Do not reintroduce one.

    _cleanup_old_main_dsp_confs

    systemctl --user daemon-reload
    for role in "${EPOS_INSTANCE_ROLES[@]}"; do
        systemctl --user enable "pipewire-epos@$role.service" 2>/dev/null || true
    done
    ok "epos instances enabled for next graphical session (eq voice sidetone)"
}

_post_install_main_restart_note() {
    banner "ONE-TIME main audio restart REQUIRED (do this when ready — it restarts your audio session briefly):"
    printf '        systemctl --user restart pipewire wireplumber\n'
    printf '        systemctl --user start  pipewire-epos@eq pipewire-epos@voice pipewire-epos@sidetone\n'
    printf '\n'
    printf '  - Removes any leftover %s from an older install.\n' "$EQ_NULL_SINK_CONF"
    printf '  - Drops the OLD daemon DSP chains (if any) from the running main instance.\n'
    printf '  - Starts EQ/voice/sidetone DSP instances (Option B, 3i).\n'
    printf '  - After this, DSP changes only restart pipewire-epos@* instances — never main.\n'
}

_install_user() {
    _need_build
    mkdir -p "$BIN_DIR"
    install -m 0755 "$ROOT/target/release/$BIN_NAME" "$BIN_DIR/$BIN_NAME"
    ok "binary -> $BIN_DIR/$BIN_NAME"

    mkdir -p "$SERVICE_DIR"
    # Rewrite ExecStart to point at the installed binary.
    sed "s|^ExecStart=.*|ExecStart=$BIN_DIR/$BIN_NAME|; \
         s|^Environment=RUST_LOG=.*|Environment=RUST_LOG=${RUST_LOG_DEFAULT}|" \
        "$ROOT/systemd/$SERVICE" > "$SERVICE_DIR/$SERVICE"
    ok "service -> $SERVICE_DIR/$SERVICE"

    _install_pw_epos

    systemctl --user daemon-reload
    systemctl --user enable --now "$SERVICE" 2>/dev/null || true
    ok "systemd user service enabled + started"

    _post_install_main_restart_note

    banner "Next: install the udev rule (one-time, needs sudo):"
    printf '        sudo ./scripts/install.sh --udev\n'
}

_install_gui() {
    # Build GUI binary if missing
    if [[ ! -x "$ROOT/target/release/$GUI_BIN" ]]; then
        banner "Building GUI (release)..."
        ( cd "$ROOT" && cargo build --release -p epos-gsx300-gui )
    fi

    mkdir -p "$BIN_DIR"
    install -m 0755 "$ROOT/target/release/$GUI_BIN" "$BIN_DIR/$GUI_BIN"
    ok "gui binary -> $BIN_DIR/$GUI_BIN"

    # Desktop entry
    mkdir -p "$APPS_DIR"
    install -m 0644 "$ROOT/packaging/$DESKTOP_FILE" "$APPS_DIR/$DESKTOP_FILE"
    update-desktop-database "$APPS_DIR" 2>/dev/null || true
    ok "desktop entry -> $APPS_DIR/$DESKTOP_FILE"

    # Icon (128x128 PNG)
    if [[ -f "$ROOT/src-tauri/icons/128x128.png" ]]; then
        # GUI project is under crates/epos-gsx300-gui/src-tauri/icons
        local ICON_SRC="$ROOT/crates/epos-gsx300-gui/src-tauri/icons/128x128.png"
        if [[ ! -f "$ICON_SRC" ]]; then
            ICON_SRC="$ROOT/src-tauri/icons/128x128.png"
        fi
        if [[ -f "$ICON_SRC" ]]; then
            mkdir -p "$ICONS_DIR/${ICON_SIZE}x${ICON_SIZE}/apps"
            install -m 0644 "$ICON_SRC" "$ICONS_DIR/${ICON_SIZE}x${ICON_SIZE}/apps/epos-gsx300.png"
            ok "icon -> $ICONS_DIR/${ICON_SIZE}x${ICON_SIZE}/apps/epos-gsx300.png"
        fi
    fi

    banner "GUI installed. Launch: $GUI_BIN or find 'EPOS GSX 300' in your app launcher."
}

_install_udev() {
    [[ "$(id -u)" -eq 0 ]] || die "udev rules need root: run with sudo"
    install -m 0644 "$ROOT/udev/$UDEV_RULE" "$UDEV_DIR/$UDEV_RULE"
    udevadm control --reload
    udevadm trigger --subsystem-match=usb --attr-match=idVendor=1395 2>/dev/null || true
    udevadm trigger --subsystem-match=hidraw --attr-match=idVendor=1395 2>/dev/null || true
    ok "udev rule installed + reloaded"
    banner "Replug the GSX 300 or reboot for the rule to take effect."
    # Ensure the current user is in the audio group (rule uses GROUP=audio).
    if command -v id >/dev/null && ! id -nG "$SUDO_USER" 2>/dev/null | tr ' ' '\n' | grep -qx audio; then
        warn "user '$SUDO_USER' is not in the audio group."
        printf '        sudo usermod -aG audio %s   (then re-login)\n' "$SUDO_USER"
    fi
}

_install_system() {
    _need_build
    install -m 0755 "$ROOT/target/release/$BIN_NAME" /usr/local/bin/$BIN_NAME
    ok "binary -> /usr/local/bin/$BIN_NAME"
    install -m 0644 "$ROOT/systemd/$SERVICE" "/usr/lib/systemd/user/$SERVICE"
    ok "service -> /usr/lib/systemd/user/$SERVICE"
    install -m 0644 "$ROOT/systemd/$EPOS_PW_SERVICE" "/usr/lib/systemd/user/$EPOS_PW_SERVICE"
    ok "instance template -> /usr/lib/systemd/user/$EPOS_PW_SERVICE"
    "$0" --udev
    systemctl --user daemon-reload
    systemctl --user enable --now "$SERVICE" 2>/dev/null || true
    for role in "${EPOS_INSTANCE_ROLES[@]}"; do
        systemctl --user enable "pipewire-epos@$role.service" 2>/dev/null || true
    done
    ok "service enabled"
    _post_install_main_restart_note
}

_uninstall() {
    systemctl --user disable --now "$SERVICE" 2>/dev/null || true
    for role in "${EPOS_INSTANCE_ROLES[@]}"; do
        systemctl --user disable --now "pipewire-epos@$role.service" 2>/dev/null || true
    done
    rm -f "$BIN_DIR/$BIN_NAME" "$SERVICE_DIR/$SERVICE" "$SERVICE_DIR/$EPOS_PW_SERVICE"
    rm -f "$PW_CONF_DIR/$EQ_NULL_SINK_CONF"
    # Remove the retired WirePlumber routing rule if an older install left one.
    rm -f "$WP_CONF_DIR/51-epos-eq-route.conf"
    rm -rf "$EPOS_CONF_DIR"
    ok "removed daemon binary + services + epos instance confs + null-sink anchor"

    rm -f "$BIN_DIR/$GUI_BIN" "$APPS_DIR/$DESKTOP_FILE"
    rm -f "$ICONS_DIR/${ICON_SIZE}x${ICON_SIZE}/apps/epos-gsx300.png"
    update-desktop-database "$APPS_DIR" 2>/dev/null || true
    ok "removed GUI binary + desktop entry + icon"

    printf '  Remove the udev rule with:  sudo rm /etc/udev/rules.d/%s && sudo udevadm control --reload\n' "$UDEV_RULE"
    rm -f "$HOME/.config/epos-gsx300/config.json" 2>/dev/null || true
    ok "removed default config (if present)"
}

if [[ $# -eq 0 ]]; then
    _install_user
else
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --udev)      _install_udev ;;
            --gui)       _install_gui ;;
            --system)    _install_system ;;
            --uninstall) _uninstall ;;
            *)           die "unknown option: $1" ;;
        esac
        shift
    done
fi

banner "Done."