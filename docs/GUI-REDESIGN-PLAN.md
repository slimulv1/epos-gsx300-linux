# GUI-REDESIGN-PLAN.md — Native Desktop App for EPOS GSX 300

> Implementation guide. Mark **`[x]`** in the checklist when a task is complete.
> Reviewed by user: **2026-09-10** (Phase A–E plan + v2.0 update from full review).
> Status: **IN PROGRESS**

---

## 📌 Goal

Convert `epos-gsx300-gui` from a web GUI (run via `npm run dev` + browser) into a **native desktop app** that runs as a single-click binary, following the architecture pattern from [sgtaziz/lian-li-linux](https://github.com/sgtaziz/lian-li-linux), with an interface **cloned from the EPOS Gaming Suite** (4 screenshots from pcper.com — stored at `/tmp/epos-ui-ref/screen-{1,2,3,4}.png`).

**Keep 100% unchanged:** daemon `epos-gsx300d`, `epos-shared`, Unix socket IPC, systemd, udev. Do not touch LED desync/heartbeat.

## 🔑 Core Facts

- The project already has a Tauri shell (`src-tauri`) + Unix socket client **but was never built into a desktop app** — the user had to open a browser via Vite.
- The GUI is currently **not a member of the Cargo workspace** → it must be added.
- `webkit2gtk-4.1 2.52.6` — **already installed** ✅ (nothing to install).
- `pw-play` available (used during DSP verification).
- Project root: `~/Projects/epos-gsx300-linux` (git HEAD `a92f2c2`).

---

## ✅ Approved — Design Decisions

| # | Decision | Value |
|---|----------|-------|
| 1 | Theme | EPOS clone: background `#1B2B30`, panel `#1A1F22`, footer `#151A1D`, teal accent `#4ECDC4`, muted `#8A9BA0`, grid `#2A3A3F` |
| 2 | EQ bands | **The daemon's real 9 bands** (64→16k) — NO fake 10th node |
| 3 | Window | **Responsive fluid** for dwm (tiling WM): full at ≥1100px, stacked at 700–1100, collapsed header <700, scrollable content at <500 height. Min 640×400. ResizeObserver redraws EQ canvas |
| 4 | Font | Inter (dark technical/premium) |
| 5 | Micro-interaction | Hover/active 150–300ms, EQ dot hover glow + freq/dB tooltip, teal focus ring, keyboard nav (Tab + ↑↓ for EQ band) |
| 6 | Added stack | Naive UI (component library) + Pinia (state) + lucide-vue-next (icons) |
| 7 | REVERBERATION slider | Maps to **sidetone level** (already in daemon) |
| 8 | SOUND TEST | Embedded WAV tone + `pw-play --target epos-eq-input` via `tauri-plugin-shell`; second press = stop |
| 9 | Native dialog | `tauri-plugin-dialog` (replaces browser prompt/confirm) for preset add |
| 10 | Mock/HTTP | **Production uses ONLY Tauri invoke → Unix socket.** Remove mock + HTTP fallback from the release path (HTTP bridge 9898 stays in the daemon) |
| 11 | Disconnected UX | Banner + disabled controls when `device_connected=false` (2s poll) |
| 12 | Settings tab | Keep STATUS (daemon/config version) + SHOW ON STARTUP (`tauri-plugin-autostart`). Remove notifications/language/social/EPOS links |
| 13 | Removed from EPOS | Help "?" button (Windows-only), Headset tab (GSX 300 has none) |
| 14 | Icon | Custom teal EPOS-style icon, separate desktop entry |

---

## 🗺️ Phases

### Phase 0 — Pre-work cleanup
- [x] Create custom icon (`src-tauri/icons/`) — teal EPOS-style (LED ring + EQ bars), `tauri icon` from original 1024×1024 PNG → generates png/icns/ico ✅ (2026-09-10)
- [x] Commit 2 dirty files: `epos-gsx300d/src/led.rs` + `main.rs` (LED always-write + heartbeat + trust readback) — ✅ commit `75e0a6b` pushed (2026-09-10)

### Phase A — Real desktop app (lian-li standard)
- [x] A1. Add `crates/epos-gsx300-gui/src-tauri` (correct path: Tauri keeps Cargo.toml inside `src-tauri/`) as a Cargo workspace member ✅ (2026-09-11)
- [x] A2. Add npm deps: `naive-ui`, `pinia`, `lucide-vue-next`, `@tauri-apps/plugin-dialog`, `@tauri-apps/plugin-shell`, `@tauri-apps/plugin-autostart` ✅ (2026-09-11)
- [x] A3. Add Rust deps: `tauri-plugin-dialog`, `tauri-plugin-shell` (already present), `tauri-plugin-autostart` — API 2.5.1: `init(launcher, Option<Vec<&str>>)` ✅ (2026-09-11)
- [x] A4. `tauri.conf.json`: 1000×630 window, min 640×400, resizable, centered, CSP null ✅ (2026-09-11)
- [x] A5. Pinia store `src/stores/daemon.ts`: connected/mode/audio/profiles/device + 3s polling, `main.ts` registers pinia ✅ (2026-09-11)
- [x] A6. Production mock/HTTP removed — new store: if `tauriInvoke` exists → invoke ONLY, failure = disconnected (no mock); mock/HTTP kept for browser-dev ✅ (2026-09-11)
- [ ] A7. Test build `cargo build --release` producing runnable `epos-gsx300-gui` binary (deferred to Phase C — waiting for complete UI)

### Phase B — EPOS Gaming Suite UI clone
- [x] B1. CSS design tokens: `--bg`, `--panel`, `--footer`, `--accent`, `--muted`, `--grid` variables, responsive spacing (clamp) ✅ (2026-09-11: global.css + test-tone WAV src-tauri/resources/epos-test-tone.wav)
- [x] B2. Layout shell: header (EPOS wordmark + device name + preset dropdown + save + status dot + min/close) + content area + 4-icon bottom-nav ✅ (2026-09-11: App.vue rewrite)
- [x] B3. Bottom nav (lucide): `[EQ sliders] [Headphones] [Mic] [Gear]` — active = teal, keyboard accessible ✅ (2026-09-11: lucide icons + active teal underline)
- [x] B4. **Playback** tab: EqCurve 9-band restyle (dotted grid, +6/−6 dB, glow, tooltip, keyboard) + preset dropdown + save button ✅ (2026-09-11: EqCurve.vue rewrite + PlaybackView.vue)
- [x] B5. Playback bottom panel: SURROUND mode toggle `2.0`/`7.1` (teal-outlined pill) + REVERBERATION→sidetone slider + SOUND TEST play/stop ✅ (2026-09-11: invoke play_test_tone + state-driven toggle)
- [x] B6. **Microphone** tab: VOICE ENHANCER (Off/Warm/Clear/Custom) + MIC GAIN slider + NOISE GATE toggle + threshold ✅ (2026-09-11: MicrophoneView.vue)
- [x] B7. **Device** tab: product card (SVG schematic of GSX 300 instead of rendered image) + Surround/2.0 state + USB/ALSA/PW/HID + smart button action dropdown ✅ (2026-09-11: DeviceView.vue)
- [x] B8. **Settings** tab: STATUS (daemon version, config version) + SHOW ON STARTUP toggle (autostart) + About ✅ (2026-09-11: SettingsView.vue + autostart_get/set commands)
- [x] B9. Disconnected banner + disabled controls (`device_connected=false`) ✅ (2026-09-11: App.vue banner + :disabled bindings in all views)
- [x] B10. EQ preset dropdown loading from daemon (`GetProfiles`) + `+` native dialog to add preset ✅ (2026-09-11: PlaybackView.vue + tauri-plugin-dialog)

### Phase C — Polish & packaging
- [x] C1. `vue-tsc --noEmit` + `cargo check --workspace` clean, 0 warnings ✅ (2026-09-11: both pass)
- [x] C2. `npm run build` (vue-tsc + vite) — production bundle ✅ (2026-09-11: 99KB JS + 15KB CSS)
- [x] C3. `cargo build --release` — complete desktop binary ✅ (2026-09-11: 1m28s clean)
- [x] C4. Desktop entry: `packaging/epos-gsx300-gui.desktop` + install to `~/.local/share/applications` + `update-desktop-database` ✅ (2026-09-11: file created + install.sh --gui)
- [x] C5. Update `scripts/install.sh` (build + install GUI + desktop entry) ✅ (2026-09-11: --gui option, uninstall cleanup)
- [ ] C6. Test binary on real machine (dwm tiled horizontal + vertical + fullscreen — responsive testing)
- [ ] C7. Screenshot real run for user review
- [x] C8. Update README (desktop app run instructions) ✅ (2026-09-11: GUI section + tech stack update)
- [ ] C9. Commit + push — **awaiting user approval (rule #24)**

---

## 🖼️ EPOS UI Reference (from vision agent)

### Screen 1 & 4 — EQ tab (FLAT / MOVIE)
- Header 60px: `EPOS` | `EPOS GSX 300` | `[FLAT ▾] [+]` | `VOLUME` | min/close
- EQ graph: 10 nodes at 64/125/250/500/1k/2k/4k/8k/16k (we use 9), Y +06→−06, dotted grid, curve glow `#4ECDC4`
- Bottom panel (2 columns): **SURROUND EFFECT** (MODE `2.0`/`7.1` pill + REVERBERATION slider) · **SOUND TEST** (AUDIO FEEDBACK + play btn)
- Movie preset = V-curve: 64:+4.5, 125:+3, 250-2k:0, 4k:+1, 8k:+4, 16k:+5.5

### Screen 2 — Microphone tab
- VOICE ENHANCER: OFF / WARM / CLEAR
- Output indicator + mic EQ display area + MIC GAIN + NOISE GATE

### Screen 3 — Settings tab
- Left: **SETTINGS** title / **STATUS** (SOFTWARE VERSION, DEVICE FIRMWARE VERSION) / DISCOVER links / HELP links
- Right: HEADSET card (EPOS GSX 300 + SMART BUTTON + product render) + NOTIFICATIONS + SHOW ON STARTUP + LANGUAGE + social icons
- Bottom nav gear active

### Bottom nav — 4 tabs
EQ sliders · Headphones · Mic · Gear — active teal `#4ECDC4`, inactive `#8A9BA0`

---

## 🚫 Out of Scope (approved for removal)

| Feature | Reason |
|---------|--------|
| Help "?" buttons around UI | Windows-only, meaningless on Linux |
| Headset tab | GSX 300 has no dedicated headset — replaced by Device tab |
| NOTIFICATIONS | Doesn't fit a Linux app |
| LANGUAGE | App is single-language EN |
| VISIT EPOS / REPORT A BUG / social icons | Third-party links — replaced by About |
| Product render images | No assets — use SVG schematic |
| LED desync / heartbeat fix | Skipped by user ("skip the led fix for later") |
| Real reverberation (surround reverb) | Daemon has none — maps to sidetone |
| Firmware update / HW 7.1 | Impossible (researched; requires vendor RE) |

---

## 📈 Progress Log

| Date | Status | Notes |
|------|--------|-------|
| 2026-09-10 | Plan approved | User approved all of Phase A–E + v2.0 update (final 3 approval comments) |
| 2026-09-10 | Phase 0: icon ✅ | Created EPOS-style icon (LED ring + EQ bars), @tauri-apps/cli v2 installed as devDependency |
| 2026-09-10 | Phase 0: commit ✅ | `75e0a6b` pushed — LED always-write + readback trust + 2s heartbeat |
| 2026-09-11 | Phase A: A1–A6 ✅ | Workspace member (src-tauri path), npm+Rust deps (autostart 2.5.1 API fix), window config, Pinia store daemon.ts (production invoke-only), main.ts pinia. `cargo check --workspace` 0 errors |
| 2026-09-11 | Phase A+B: full build ✅ | Added main.rs: `play_test_tone` (pw-play + child), `autostart_get/set`. `vue-tsc` clean + `npm run build` 99KB JS + `cargo check` 0 errors. Removed 5 old components + useDaemon.ts (migrated to store). |
| 2026-09-11 | Phase B: B1–B10 ✅ | global.css tokens, test-tone WAV, App.vue shell + 4 views (Playback/Microphone/Device/Settings) + EqCurve dotted-grid+glow+tooltip+keyboard+ResizeObserver. All new components use the Pinia store. |
| 2026-09-11 | Phase C: C1–C5, C8 ✅ | C3 clean release build, C4 desktop entry, C5 install.sh --gui, C8 README GUI section + tech stack. |
| 2026-09-11 | ⚠️ Daemon disabled | User disabled the systemd epos-gsx300d — filter-chain DSP caused headache-inducing interference (annoying buzzing/humming). Code + static review only, NO live runs. |