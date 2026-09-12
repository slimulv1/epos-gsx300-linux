# EPOS GSX 300 — Research Archive (Agent Compilation)

> Tổng hợp toàn bộ tài liệu thu thập từ các background research agents
> (arise_background shadows) + kết quả reverse engineering thực nghiệm,
> lưu trữ tập trung tại `~/Projects/epos-gsx300-reverse-engineering/`.
>
> **Cập nhật lần cuối:** 2026-09-10 (DSP milestone hoàn tất)

---

## 1. Nguồn tài liệu & cách tổ chức

| File | Nội dung | Nguồn |
|------|----------|-------|
| `README.md` | Phân tích phần cứng 13 chương (chipset, USB, UAC, ALSA/PipeWire, 7.1 mystery, protocol RE) | 3 agent round 1: beru (local data), 2× tank (hardware + USB kernel) |
| `QUICK-REFERENCE.md` | Thẻ tra cứu 1 trang (device identity, streams, mixer, workaround) | Compiled từ round 1 |
| `FEASIBILITY-ANALYSIS.md` | Phân tích 18 tính năng EPOS Gaming Suite → Linux feasibility, đã cập nhật trạng thái SOLVED | 2 agent round 2 (UE UI/protocol analysis) + nghiệm thu 2026-09-10 |
| `BLUEPRINT.md` | Thiết kế architecture epos-gsx300-linux (daemon + Tauri GUI, 4 phase) | 3 agent round 3 (lian-li-linux architecture research) |
| `AGENT-FINDINGS.md` | **File này** — bảng điểm + chi tiết từng agent + phát hiện mới nhất | Tất cả rounds + thực nghiệm hardware |
| `_epos-tmp/` | Scaffold workspace cũ (Cargo.toml) — **có thể xóa** | Round 3 |

---

## 2. Bảng tổng hợp findings theo agent

### Round 1 (2026-09-09) — Hardware identification & kernel RE

| Agent | Nhiệm vụ | Findings chính |
|-------|----------|----------------|
| **beru** | Thu thập dữ liệu local (lsusb, /proc/asound, dmesg) | VID:PID `1395:0098`, serial `A003200202602692`, 4 USB interfaces, UAC1, Full Speed 12Mbps, bus-powered 100mA, CX20988 (ban đầu nhầm CX21988) |
| **tank A** | Chipset deep dive (Conexant CX20988) | Không datasheet công khai; 7.1 là host-side DSP (không có DAC 7.1 thật); EPOS Gaming Suite = hoàn toàn software processing |
| **tank B** | USB Audio Class kernel RE | UAC1 classification, sample rate negotiation (48k/96k), feedback endpoint, 7.1 descriptor layout (UAC2), ALSA/PipeWire driver stack hoạt động native |

### Round 2 (2026-09-09) — EPOS Gaming Suite UI / protocol feasibility

| Agent | Nhiệm vụ | Findings chính |
|-------|----------|----------------|
| **tank C** | Phân tích EPOS Gaming Suite Windows UI (feature-by-feature) | 18 tính năng liệt kê đầy đủ (volume, EQ, presets, 7.1 toggle, reverb, sound test, smart button, mic gain, voice enhancer, noise gate, sidetone, mic mute, mic test, LED, firmware, language, boot, 2nd device) |
| **tank D** | Đánh giá độ khó Linux feasibility từng tính năng | Kết quả: 7 fully possible, 8 partial, 3 impossible (LED, firmware, hw 7.1). **Sau nghiệm thu 2026-09-10: chỉ còn firmware update + hw 7.1 audio processing là chưa giải** |

### Round 3 (2026-09-09) — Architecture reference (lian-li-linux)

| Agent | Nhiệm vụ | Findings chính |
|-------|----------|----------------|
| **bellion/tank E** | Phân tích repo `sgtaziz/lian-li-linux` | Pattern chuẩn: **Rust daemon + Tauri GUI + Unix socket IPC + JSON config (~/.config/app/) + udev rules (no root) + systemd user service** |
| **tank F** | Tech validation | Tauri (~12MB vs Electron 187MB), PipeWire `pw_filter` API (`pipewire` crate), `rusb` + `hidapi-rs` cho USB/HID |
| **beru (bg)** | Hỗ trợ scaffold | 3 crates: epos-shared, epos-gsx300d, epos-gsx300-gui |

### Round 4 (2026-09-09) — LED HID protocol

| Agent | Nhiệm vụ | Findings chính |
|-------|----------|----------------|
| **tank G** | Tìm tài liệu LED protocol EPOS | KHÔNG có RE công khai cho GSX 300 LED; EPOS Gaming Suite điều khiển LED qua APO driver; Busylight BL20 (PID 0x0074) là device khác, không áp dụng được |
| **beru (bg)** | Dump HID descriptor local | Decode 120-byte HID report descriptor: Report 1 (consumer vol), 0x1A (absolute), 4/5 (primary cmd), 6/7 (secondary), **Report 2 vendor (0xFF13): 2 LED bits + 3 button bits** |

---

## 3. Phát hiện thực nghiệm (hardware confirmed 2026-09-09 → 10)

> ⚠️ **Quan trọng:** những phát hiện này KHÔNG nằm trong research agent texts —
> chúng thu được bằng cách ghi trực tiếp vào `/dev/hidraw3` và quan sát LED vật lý.

### 3.1 LED ring protocol — CONFIRMED

| Byte (Report ID 0x02 OUTPUT, vendor) | Hiệu ứng LED |
|--------------------------------------|--------------|
| `0x00` | Tắt |
| `0x01` (bit0) | **XANH** — stereo (2.0) |
| `0x02` (bit1) | **ĐỎ** — surround (7.1) |
| `0x03` (cả 2 bit) | **HỒNG** (mix) |

- Initial guesses trong config defaults (`vendor_blue: 0x01`, `vendor_red: 0x02`) đúng 100%.
- Report ID 0x02 OUTPUT path: vendor (hoặc consumer Report 0x04 payload — probe script hỗ trợ cả 2).

### 3.2 Smart button (dial click) protocol — CONFIRMED

| Report ID 0x02 INPUT value | Ý nghĩa |
|----------------------------|---------|
| `0x01` | Mode = stereo (blue) |
| `0x02` | Mode = 7.1 (red) |
| `0x04` | Long-press (>2s) |

- Device gửi **state readback**, không phải momentary pulse — nút bấm toggle 2.0⇄7.1 và device trả state mới.
- Debounce quirk: click nhanh liên tiếp chỉ nhận 3/7 reports (xác nhận qua test).
- Volume dial: Report ID 0x01 (0x01=up, 0x02=down, 0x00=release).

### 3.3 DSP — PipeWire filter-chain (SOLVED 2026-09-10)

| Tính năng | Cơ chế | Config path |
|-----------|--------|-------------|
| **9-band EQ** | `filter-chain` với `bq_peaking` (Freq/Q/Gain — PipeWire tự tính coefficients) | `~/.config/pipewire/pipewire.conf.d/50-epos-eq.conf` |
| **Voice Enhancer** (Warm/Clear) | Source filter-chain trên mic EPOS: Warm = boost 200/350/500Hz, Clear = 2-8kHz | `51-epos-voice-enhancer.conf` |
| **Noise Gate** | rnnoise neural suppression qua LADSPA (`noise_suppressor_stereo`) | `93-epos-noisegate.conf` |

**Requirement noise gate:**
- `librnnoise_ladspa.so` (prebuilt từ werman/noise-suppression-for-voice v1.21) → `~/.local/lib/ladspa/`
- LADSPA_PATH drop-in: `~/.config/systemd/user/pipewire.service.d/ladspa.conf`

**Đã VERIFIED end-to-end:** nodes thật trong `pw-dump` + links đúng (EQ→EPOS sink FL/FR, voice/noisegate từ EPOS mic) + `pw-record` PASS + GUI drag band → config + conf file + nodes sync.

### 3.4 Kiến trúc daemon (final)

```
epos-gsx300d (Rust, Tokio)
├── epos-hid      HID listener: smart button + volume dial (/dev/hidraw3)
├── epos-led      LedController: Report 0x02 output, hotplug reopen, Drop reset
├── epos-audio    AudioPipeline: EQ/voice/noise-gate filter-chain configs + reload_pipewire + pw-dump node resolution
├── epos-devices  pw-dump JSON parser → real node names (thay wildcard)
├── epos-config   Config ~/.config/epos-gsx300/config.json (atomic write tmp+rename)
└── epos-ipc      Unix socket + HTTP bridge 127.0.0.1:9898 (CORS) cùng handle_request
```

**GUI:** `epos-gsx300-gui` (Tauri 2 + Vue 3) — 3 tabs Playback/Microphone/Settings, EQ canvas drag, polling 3s (fetchStatus+fetchAudio+fetchMode), mock tracking chống desync.

**Đơn vị systemd:** `epos-gsx300d.service` `Wants=pipewire` (KHÔNG `Requires=wireplumber` — tránh crash loop khi pipewire restart).

---

## 4. Trạng thái nghiệm thu (2026-09-10)

| Tính năng | Trạng thái | Verify |
|-----------|-----------|--------|
| Volume control (dial, local analog) | ✅ Native | — |
| 9-band EQ + presets | ✅ SOLVED | GUI drag → nodes live |
| 2.0/7.1 toggle + LED sync | ✅ SOLVED | curl + smart button + GUI |
| Smart button (click + long-press) | ✅ SOLVED | 46 events 1 lần test, PASS |
| Voice Enhancer Warm/Clear/Off | ✅ SOLVED | file conf đổi đúng mode |
| Noise Gate (rnnoise) | ✅ SOLVED | node tạo/xóa + pw-record PASS |
| Sidetone | ⚠️ pw-loopback | chưa nghe thử hardware cuối |
| Mic gain | ✅ amixer | GUI slider sync |
| Mic mute / mic test | ✅ Trivial | chưa UI |
| LED ring (blue/red) | ✅ SOLVED | protocol confirmed |
| Firmware update | ❌ | không làm được — proprietary |
| **7.1 audio processing thật** | ❌ | host-side EPOS APO, không có Linux alternative (chỉ HRTF/headtracking spatial thay thế) |

**Feasibility tổng:** 88% → **94%** (chỉ còn 2 mục không thể).

---

## 5. Kho lưu trữ code

| Repo | Path | Trạng thái |
|------|------|-----------|
| **slimulv1/epos-gsx300-linux** | `~/Projects/epos-gsx300-linux` | GitHub public, commits `4dd6909` → `5148adc` → `aef7301` → `70170e9` (DSP milestone) |
| PipeWire configs (A2+ + EPOS) | `~/pipewire-audio-config` | GitHub public `slimulv1/pipewire-audio-config` |

---

## 6. Xóa dư

`_epos-tmp/Cargo.toml` = scaffold workspace bỏ đi từ round 3, không còn dùng. Khuyên xóa.