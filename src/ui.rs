use eframe::egui::InputState;
use eframe::egui::Key;
pub struct Chord {
    key: Option<Key>,
    /// Ctrl, Command, Shift, Alt
    modifiers: [Option<bool>; 4],
}

impl Chord {
    pub fn new() -> Self {
        Chord {
            key: None,
            modifiers: [None, None, None, None],
        }
    }
    pub fn matches(self, input_state: &InputState) -> bool {
        if let Some(key) = self.key {
            input_state.key_pressed(key)
                && self.modifiers[0]
                    .map(|m| m == input_state.modifiers.ctrl)
                    .unwrap_or(true)
                && self.modifiers[1]
                    .map(|m| m == input_state.modifiers.command)
                    .unwrap_or(true)
                && self.modifiers[2]
                    .map(|m| m == input_state.modifiers.shift)
                    .unwrap_or(true)
                && self.modifiers[3]
                    .map(|m| m == input_state.modifiers.alt)
                    .unwrap_or(true)
        } else {
            false
        }
    }
    pub fn key(mut self, key: Key) -> Self {
        self.key.replace(key);
        self
    }
    pub fn ctrl(mut self) -> Self {
        self.modifiers[0].replace(true);
        self
    }
    pub fn command(mut self) -> Self {
        self.modifiers[1].replace(true);
        self
    }
    pub fn shift(mut self) -> Self {
        self.modifiers[2].replace(true);
        self
    }
    pub fn alt(mut self) -> Self {
        self.modifiers[3].replace(true);
        self
    }
}
