use tracing::info;

/// HID event handler for volume knob and smart button
pub struct HidHandler {
    // TODO: inotify/evdev watcher for /dev/input/event*
}

impl HidHandler {
    pub fn new() -> Self {
        Self {}
    }

    /// Handle a volume knob event
    #[allow(dead_code)]
    pub fn handle_volume_event(&self, direction: i32) {
        // Volume knob sends relative events
        // Positive = volume up, Negative = volume down
        info!("Volume knob: {}", if direction > 0 { "up" } else { "down" });
        // TODO: sync with PipeWire volume
    }

    /// Handle a smart button press
    #[allow(dead_code)]
    pub fn handle_button_press(&self) {
        info!("Smart button pressed");
        // TODO: execute configured action
    }
}
