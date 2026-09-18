#!/usr/bin/env bash
# EPOS GSX 300 — lần restart MAIN cuối cùng (một lần), sau đó chuyển sang
# per-role instances (Option B, 3i).
#
# Script này là BƯỚC 1 trong migration: restart main MỘT lần để nạp anchor
# fail-closed (null-sink `epos-eq-input`) + drop các DSP chain daemon cũ đang
# nằm trong main. Từ đây về sau mọi thay đổi DSP chỉ chạm pipewire-epos@* —
# main không bao giờ bị động vào lúc runtime.
#
# Chạy khi bạn sẵn sàng (sẽ mất âm thanh một nhịp ngắn trong lúc main
# restart). Không cần root; user-level.
#
#   ./scripts/restart-epos-session.sh
#
# Lệnh này KHÔNG được gọi tự động bởi install.sh/daemon — bạn chủ động chạy.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANCHOR_SINK="epos-eq-input"
INSTANCE_BASE="pipewire-epos@"
ROLES=( eq voice sidetone )

banner() { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
ok()     { printf '\033[1;32m  ✔\033[0m %s\n' "$*"; }
warn()   { printf '\033[1;33m  ⚠\033[0m %s\n' "$*" >&2; }
die()    { printf '\033[1;31m✖\033[0m %s\n' "$*" >&2; exit 1; }

# Anchor đã xuất hiện trong MAIN chưa? (null-sink epos-eq-input).
_anchor_up() {
    pactl list short sinks 2>/dev/null | grep -q "$ANCHOR_SINK" ||
        pactl list short sources 2>/dev/null | grep -q "$ANCHOR_SINK"
}

# Instance pipewire-epos@<role> có control socket riêng đang trả lời chưa?
_instance_ok() {
    pw-cli -r "pipewire-epos-$1" info 0 >/dev/null 2>&1
}

die() { printf '\033[1;31m✖\033[0m %s\n' "$*" >&2; exit 1; }

_main_anchor_note() { # printed once; explains the only-involved main restart
    banner "ĐANG restart MAIN lần cuối (audio session sẽ mất ~vài giây)..."
}

if [[ ! -x "$ROOT/scripts/install.sh" ]]; then
    die "install.sh missing — run ./scripts/install.sh --help trước"
fi
# Template instance unit phải đã được cài bởi install.sh. Lưu ý:
# systemd user KHÔNG liệt kê instance alias (pipewire-epos@eq/voice/..) trong
# list-unit-files — chỉ template `pipewire-epos@.service` (state indirect).
# Nên gate đúng: template có + 3 symlink enable (wants) trên disk tồn tại.
if ! systemctl --user list-unit-files "pipewire-epos@.service" >/dev/null 2>&1; then
    die "chưa thấy template pipewire-epos@.service — chạy ./scripts/install.sh trước rồi restart main"
fi
for role in "${EPOS_INSTANCE_ROLES[@]}"; do
    if [[ ! -e "$XDG_CONFIG_HOME/systemd/user/graphical-session.target.wants/pipewire-epos@$role.service" ]] &&
       [[ ! -e "$HOME/.config/systemd/user/graphical-session.target.wants/pipewire-epos@$role.service" ]]; then
        die "chưa thấy enable symlink pipewire-epos@$role — chạy ./scripts/install.sh trước rồi restart main"
    fi
done

banner "Restart MAIN lần đầu (nạp null-sink fail-closed + drop DSP chain cũ)"
systemctl --user restart pipewire wireplumber

banner "Chờ anchor $ANCHOR_SINK lên trong MAIN (fail-closed)..."
deadline=$((SECONDS + 20))
until _anchor_up; do
    if (( SECONDS >= deadline )); then
        warn "anchor $ANCHOR_SINK chưa thấy — kiểm tra 40-epos-eq-virtualsink.conf ở main"
        die "fail-closed: EPOS im lặng (an toàn), KHÔNG tự fallback"
    fi
    sleep 0.5
done
ok "anchor $ANCHOR_SINK đã lên trong main"

banner "Khởi động 3 instance DSP (eq / voice / sidetone)"
for role in "${ROLES[@]}"; do
    systemctl --user restart "${INSTANCE_BASE}${role}.service"
    ok "restarted pipewire-epos@$role"
done

banner "Health-check từng instance qua control socket riêng"
for role in "${ROLES[@]}"; do
    if _instance_ok "$role"; then
        ok "epos instance $role healthy"
    else
        warn "epos instance $role chưa trả lời — EPOS path im lặng (fail-closed); retry: systemctl --user restart pipwire-epos@$role"
    fi
done

banner "Xong — Option B (3i) đã live. Ghi nhớ:"
printf '  - Mọi thay đổi DSP (EQ/voice/noise/sidetone) từ giờ chỉ restart các\n'
printf '    instance pipewire-epos@eq|voice|sidetone — main KHÔNG đụng vào nữa.\n'
printf '  - App EPOS target cố định vào %s (anchor tĩnh) nên fail-closed.\n' "$ANCHOR_SINK"
printf '  - Bước route WirePlumber (51-epos-eq-route.conf) vẫn INACTIVE — đánh\n'
printf '    giá ở phase validation trước khi bật (xem header conf).\n'
