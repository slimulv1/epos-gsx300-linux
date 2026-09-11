# GUI-REDESIGN-PLAN.md — Native Desktop App cho EPOS GSX 300

> File hướng dẫn triển khai. **Checklist đánh dấu `[x]` khi task hoàn thành.**
> Duyệt bởi user: **2026-09-10** (kế hoạch Phase A–E + cập nhật v2.0 từ kiểm tra tổng thể).
> Trạng thái: **ĐANG TRIỂN KHAI**

---

## 📌 Mục tiêu

Chuyển `epos-gsx300-gui` từ web GUI (chạy `npm run dev` + browser) thành **desktop app native** chạy bằng binary 1 click, theo chuẩn kiến trúc [sgtaziz/lian-li-linux](https://github.com/sgtaziz/lian-li-linux), giao diện **clone EPOS Gaming Suite** (4 screenshot từ pcper.com — lưu tại `/tmp/epos-ui-ref/screen-{1,2,3,4}.png`).

**Giữ nguyên 100%:** daemon `epos-gsx300d`, `epos-shared`, Unix socket IPC, systemd, udev. Không đụng LED desync/heartbeat.

## 🔑 Sự thật cốt lõi

- Project đã có Tauri shell (`src-tauri`) + Unix socket client **nhưng chưa bao giờ build thành desktop app** — user phải mở browser qua Vite.
- GUI hiện **không phải member của Cargo workspace** → phải thêm.
- `webkit2gtk-4.1 2.52.6` — **đã cài sẵn** ✅ (không cần cài gì).
- `pw-play` khả dụng (đã dùng trong DSP verification).
- Project root: `~/Projects/epos-gsx300-linux` (git HEAD `a92f2c2`).

---

## ✅ Đã duyệt — Các quyết định thiết kế

| # | Quyết định | Giá trị |
|---|-----------|--------|
| 1 | Theme | Clone EPOS: nền `#1B2B30`, panel `#1A1F22`, footer `#151A1D`, accent teal `#4ECDC4`, muted `#8A9BA0`, grid `#2A3A3F` |
| 2 | EQ bands | **9 band thật của daemon** (64→16k) — KHÔNG thêm node ảo thứ 10 |
| 3 | Window | **Responsive fluid** cho dwm (tiling WM): ≥1100px đầy đủ, 700–1100 dọc, <700 thu gọn header, chiều cao <500 scroll content. Min 640×400. ResizeObserver redraw EQ canvas |
| 4 | Font | Inter (dark technical/premium) |
| 5 | Micro-interaction | Hover/active 150–300ms, EQ dot hover glow + tooltip freq/dB, focus ring teal, keyboard nav (Tab + ↑↓ EQ band) |
| 6 | Stack thêm | Naive UI (component library) + Pinia (state) + lucide-vue-next (icons) |
| 7 | REVERBERATION slider | Map sang **sidetone level** (daemon có sẵn) |
| 8 | SOUND TEST | WAV tone embed + `pw-play --target epos-eq-input` qua `tauri-plugin-shell`; bấm lần 2 = stop |
| 9 | Native dialog | `tauri-plugin-dialog` (thay browser prompt/confirm) cho preset add |
| 10 | Mock/HTTP | Bản **production CHỈ dùng Tauri invoke → Unix socket**. Bỏ mock + HTTP fallback khỏi release path (HTTP bridge 9898 giữ nguyên trong daemon) |
| 11 | Disconnected UX | Banner + disable controls khi `device_connected=false` (poll 2s) |
| 12 | Settings tab | Giữ STATUS (daemon/config version) + SHOW ON STARTUP (`tauri-plugin-autostart`). Bỏ notifications/language/social/EPOS links |
| 13 | Bỏ khỏi EPOS | Nút Help "?" (Windows-only), tab Headset (GSX 300 không có) |
| 14 | Icon | Custom teal EPOS-style icon, desktop entry riêng |

---

## 🗺️ Phases

### Phase 0 — Dọn dẹp trước khi bắt đầu
- [x] Tạo custom icon (`src-tauri/icons/`) — teal EPOS-style (LED ring + EQ bars), `tauri icon` từ PNG gốc 1024×1024 → sinh đủ png/icns/ico ✅ (2026-09-10)
- [x] Commit 2 file dirty: `epos-gsx300d/src/led.rs` + `main.rs` (LED always-write + heartbeat + trust readback) — ✅ commit `75e0a6b` đã push (2026-09-10)

### Phase A — Desktop app thật (chuẩn lian-li)
- [x] A1. Thêm `crates/epos-gsx300-gui/src-tauri` (path đúng: Tauri giữ Cargo.toml trong `src-tauri/`) làm member của Cargo workspace ✅ (2026-09-11)
- [x] A2. Thêm deps npm: `naive-ui`, `pinia`, `lucide-vue-next`, `@tauri-apps/plugin-dialog`, `@tauri-apps/plugin-shell`, `@tauri-apps/plugin-autostart` ✅ (2026-09-11)
- [x] A3. Thêm Rust deps: `tauri-plugin-dialog`, `tauri-plugin-shell` (có sẵn), `tauri-plugin-autostart` — API 2.5.1: `init(launcher, Option<Vec<&str>>)` ✅ (2026-09-11)
- [x] A4. `tauri.conf.json`: 1000×630 window, min 640×400, resizable, center, CSP null ✅ (2026-09-11)
- [x] A5. Pinia store `src/stores/daemon.ts`: connected/mode/audio/profiles/device + polling 3s, `main.ts` đăng ký pinia ✅ (2026-09-11)
- [x] A6. Production mock/HTTP bỏ — store mới: nếu `tauriInvoke` tồn tại → CHỈ invoke, fail = disconnected (không mock); mock/HTTP giữ cho browser-dev ✅ (2026-09-11)
- [ ] A7. Build thử `cargo build --release` ra binary `epos-gsx300-gui` chạy được (dồn Phase C — chờ UI hoàn chỉnh)

### Phase B — Giao diện clone EPOS Gaming Suite
- [x] B1. CSS design tokens: biến `--bg`, `--panel`, `--footer`, `--accent`, `--muted`, `--grid`, spacing responsive (clamp) ✅ (2026-09-11: global.css + test-tone WAV src-tauri/resources/epos-test-tone.wav)
- [x] B2. Layout shell: header (EPOS wordmark + device name + preset dropdown + save + status dot + min/close) + content area + bottom-nav 4 icon ✅ (2026-09-11: App.vue rewrite)
- [x] B3. Bottom nav (lucide): `[EQ sliders] [Headphones] [Mic] [Gear]` — active = teal, keyboard accessible ✅ (2026-09-11: lucide icons + active teal underline)
- [x] B4. Tab **Playback**: EqCurve 9-band restyle (grid dotted, +6/−6 dB, glow, tooltip, keyboard) + preset dropdown + save button ✅ (2026-09-11: EqCurve.vue rewrite + PlaybackView.vue)
- [x] B5. Bottom panel Playback: SURROUND mode toggle `2.0`/`7.1` (pill teal border) + REVERBERATION→sidetone slider + SOUND TEST play/stop ✅ (2026-09-11: invoke play_test_tone + state-driven toggle)
- [x] B6. Tab **Microphone**: VOICE ENHANCER (Off/Warm/Clear/Custom) + MIC GAIN slider + NOISE GATE toggle + threshold ✅ (2026-09-11: MicrophoneView.vue)
- [x] B7. Tab **Device**: product card (SVG schematic GSX 300 thay ảnh render) + trạng thái Surround/2.0 + USB/ALSA/PW/HID + smart button action dropdown ✅ (2026-09-11: DeviceView.vue)
- [x] B8. Tab **Settings**: STATUS (daemon version, config version) + SHOW ON STARTUP toggle (autostart) + licencia About ✅ (2026-09-11: SettingsView.vue + autostart_get/set commands)
- [x] B9. Disconnected banner + disable controls (`device_connected=false`) ✅ (2026-09-11: App.vue banner + :disabled binding in all views)
- [x] B10. EQ preset dropdown load từ daemon (`GetProfiles`) + `+` native dialog add preset ✅ (2026-09-11: PlaybackView.vue + tauri-plugin-dialog)

### Phase C — Hoàn thiện & đóng gói
- [x] C1. `vue-tsc --noEmit` + `cargo check --workspace` sạch 0 warning ✅ (2026-09-11: both pass)
- [x] C2. `npm run build` (vue-tsc + vite) — production bundle ✅ (2026-09-11: 99KB JS + 15KB CSS)
- [x] C3. `cargo build --release` — desktop binary hoàn chỉnh ✅ (2026-09-11: 1m28s clean)
- [x] C4. Desktop entry: `packaging/epos-gsx300-gui.desktop` + cài vào `~/.local/share/applications` + `update-desktop-database` ✅ (2026-09-11: file created + install.sh --gui)
- [x] C5. Cập nhật `scripts/install.sh` (build + cài GUI + desktop entry) ✅ (2026-09-11: --gui option, uninstall cleanup)
- [ ] C6. Chạy thử binary trên máy thật (dwm tile ngang + dọc + fullscreen — test responsive)
- [ ] C7. Chụp screenshot chạy thật trình user
- [x] C8. Update README (hướng dẫn chạy desktop app) ✅ (2026-09-11: GUI section + tech stack update)
- [ ] C9. Commit + push — **chờ user duyệt (rule #24)**

---

## 🖼️ Tham chiếu giao diện EPOS (từ vision agent)

### Screen 1 & 4 — EQ tab (FLAT / MOVIE)
- Header 60px: `EPOS` | `EPOS GSX 300` | `[FLAT ▾] [+]` | `VOLUME` | min/close
- EQ graph: 10 node 64/125/250/500/1k/2k/4k/8k/16k (ta dùng 9), Y +06→−06, grid dotted, curve glow `#4ECDC4`
- Bottom panel (2 cột): **SURROUND EFFECT** (MODE `2.0`/`7.1` pill + REVERBERATION slider) · **SOUND TEST** (AUDIO FEEDBACK + play btn)
- Movie preset = V-curve: 64:+4.5, 125:+3, 250-2k:0, 4k:+1, 8k:+4, 16k:+5.5

### Screen 2 — Microphone tab
- VOICE ENHANCER: OFF / WARM / CLEAR
- Output chỉ thị + mic EQ vùng hiển thị + MIC GAIN + NOISE GATE

### Screen 3 — Settings tab
- Left: **SETTINGS** title / **STATUS** (SOFTWARE VERSION, DEVICE FIRMWARE VERSION) / DISCOVER links / HELP links
- Right: HEADSET card (EPOS GSX 300 + SMART BUTTON + product render) + NOTIFICATIONS + SHOW ON STARTUP + LANGUAGE + social icons
- Bottom nav gear active

### Bottom nav 4 tab
EQ sliders · Headphones · Mic · Gear — active teal `#4ECDC4`, inactive `#8A9BA0`

---

## 🚫 Ngoài phạm vi (đã duyệt bỏ)

| Chức năng | Lý do |
|-----------|-------|
| Nút Help "?" quanh UI | Windows-only, vô nghĩa trên Linux |
| Tab Headset | GSX 300 không có headset riêng — thay bằng tab Device |
| NOTIFICATIONS | Không khớp Linux app |
| LANGUAGE | App 1 ngôn ngữ EN |
| VISIT EPOS / REPORT A BUG / social icons | App bên thứ 3 — thay bằng About |
| Ảnh render product | Không có asset — dùng SVG schematic |
| LED desync / heartbeat fix | User bỏ qua ("bỏ qua phần led fix sau") |
| Reverberation thật (surround reverb) | Daemon không có — map sang sidetone |
| Firmware update / HW 7.1 | Bất khả thi (đã research, cần vendor RE) |

---

## 📈 Nhật ký tiến độ

| Ngày | Trạng thái | Ghi chú |
|------|-----------|---------|
| 2026-09-10 | Kế hoạch duyệt | User duyệt toàn bộ Phase A–E + cập nhật v2.0 (3 câu duyệt cuối) |
| 2026-09-10 | Phase 0: icon ✅ | Tạo icon EPOS-style (LED ring + EQ bars), @tauri-apps/cli v2 cài làm devDependency |
| 2026-09-10 | Phase 0: commit ✅ | `75e0a6b` push — LED always-write + readback trust + 2s heartbeat |
| 2026-09-11 | Phase A: A1–A6 ✅ | Workspace member (src-tauri path), deps npm+Rust (autostart 2.5.1 API fix), window config, Pinia store daemon.ts (production chỉ invoke), main.ts pinia. `cargo check --workspace` 0 lỗi |
| 2026-09-11 | Phase A+B: full build ✅ | Thêm main.rs: `play_test_tone` (pw-play + child), `autostart_get/set`. `vue-tsc` clean + `npm run build` 99KB JS + `cargo check` 0 lỗi. Xóa 5 component cũ + useDaemon.ts (đã chuyển sang store). |
| 2026-09-11 | Phase B: B1–B10 ✅ | global.css tokens, test-tone WAV, App.vue shell + 4 views (Playback/Microphone/Device/Settings) + EqCurve dotted-grid+glow+tooltip+keyboard+ResizeObserver. Tất cả component mới dùng Pinia store. |
| 2026-09-11 | Phase C: C1–C5, C8 ✅ | C3 release build clean, C4 desktop entry, C5 install.sh --gui, C8 README GUI section + tech stack. |
| 2026-09-11 | ⚠️ Daemon disabled | User tắt systemd epos-gsx300d — filter-chain DSP gây nhiễu "ù ù đau đầu". Chỉ code + review tĩnh, KHÔNG chạy thực tế. |