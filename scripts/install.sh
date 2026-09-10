#!/usr/bin/env bash
# Install EPOS GSX 300 daemon + udev rules + systemd user service.
#
# Usage:
#   ./scripts/install.sh              install daemon + service (user)
#   ./scripts/install.sh --udev       install udev rule (needs sudo)
#   ./scripts/install.sh --system     system-wide install into /usr/local
#   ./scripts/install.sh --uninstall  remove everything installed by this script
#
# The GUI (Tauri) is built separately via:  ./scripts/build-gui.sh

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_NAME="epos-gsx300d"
UDEV_RULE="70-epos-gsx300.rules"
SERVICE="epos-gsx300d.service"

# --- paths ---------------------------------------------------------------
PREFIX="${PREFIX:-$HOME/.local}"
BIN_DIR="$PREFIX/bin"
SERVICE_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user"
UDEV_DIR="/etc/udev/rules.d"
RUST_LOG_DEFAULT="${RUST_LOG_DEFAULT:-info}"

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

    systemctl --user daemon-reload
    systemctl --user enable --now "$SERVICE" 2>/dev/null || true
    ok "systemd user service enabled + started"

    banner "Next: install the udev rule (one-time, needs sudo):"
    printf '        sudo ./scripts/install.sh --udev\n'
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
    "$0" --udev
    systemctl --user daemon-reload
    systemctl --user enable --now "$SERVICE" 2>/dev/null || true
    ok "service enabled"
}

_uninstall() {
    systemctl --user disable --now "$SERVICE" 2>/dev/null || true
    rm -f "$BIN_DIR/$BIN_NAME" "$SERVICE_DIR/$SERVICE"
    ok "removed user binary + service"
    printf '  Remove the udev rule with:  sudo rm /etc/udev/rules.d/%s && sudo udevadm control --reload\n' "$UDEV_RULE"
    rm -f "$HOME/.config/epos-gsx300/config.json" 2>/dev/null || true
    ok "removed default config (if present)"
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --udev)      _install_udev ;;
        --system)    _install_system ;;
        --uninstall) _uninstall ;;
        *)           _install_user ;;
    esac
    shift
done

[[ $# -eq 0 ]] && _install_user

banner "Done."