// Desktop notification helper for EPOS smart-button state changes.
//
// When the physical smart button is pressed the daemon bumps a monotonic
// `smart_button_seq`; the GUI notices the seq move on its 3s poll and calls
// `notifySmartButton` to surface "PROFILE · MODE" (e.g. "Flat · 7.1").
//
// Delivery strategy (best-effort, never blocks the UI):
//   1. Web Notification API (works in the Tauri WebView and plain browsers).
//   2. `notify-send` fallback through the Tauri shell plugin — this is what
//      makes it a true *OS desktop* notification even when the WebView's
//      notification permission is denied or unavailable.
//
// Notifications must fail silently: they decorate state sync, they never gate
// it ~~ if any path errors, the GUI keeps working.

import { Command } from "@tauri-apps/plugin-shell";

/**
 * Map an audio mode to its short display label ("7.1" / "Stereo"),
 * matching how the UI labels modes everywhere else.
 */
function modeLabel(mode: string | undefined): string {
  return mode === "surround71" ? "7.1" : "Stereo";
}

/**
 * Show a desktop notification carrying the current profile + mode, e.g.
 * "EPOS GSX 300 — Flat · 7.1". Best-effort: returns a promise that resolves
 * after the primary path (or rejects/falls through to notify-send).
 */
export async function notifySmartButton(profile: string, mode: string | undefined): Promise<void> {
  const title = "EPOS GSX 300";
  const body = `${profile || "Flat"} · ${modeLabel(mode)}`;
  const tag = "epos-smart-button";

  // 1) Primary: Web Notification API.
  try {
    if (typeof window !== "undefined" && "Notification" in window) {
      if (Notification.permission === "granted") {
        new Notification(title, { body, tag });
        return;
      }
      if (Notification.permission === "default") {
        const granted = await Notification.requestPermission();
        if (granted === "granted") {
          new Notification(title, { body, tag });
          return;
        }
      }
    }
  } catch {
    // Fall through to notify-send.
  }

  // 2) Fallback: notify-send via Tauri shell plugin.
  try {
    const child = await Command.create("notify-send", [
      "-u",
      "normal",
      "-a",
      "epos-gsx300",
      title,
      body,
    ]).execute();
    if (child.code !== 0) {
      console.warn(`notify-send exited with code ${child.code}`);
    }
  } catch (e) {
    // No desktop notification path available — silently ignore.
    console.debug("epos notification unavailable:", e);
  }
}
