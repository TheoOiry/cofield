use chrono::{DateTime, Local};

const NUMBER_MODE_VALUE: u8 = 27;
const BACKSPACE_VALUE: u8 = 28;
const DOT_VALUE: u8 = 29;
const SPACE_VALUE: u8 = 30;

const ALPHABET: [&str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];

#[derive(PartialEq, Eq, Copy, Clone)]
enum WritingMode {
    Letters,
    Numbers,
}

pub struct TextPattern {
    last_hand: [bool; 5],

    current_value: Option<u8>,
    current_mode: WritingMode,

    max_ms_delay: u32,
    last_moved_time: DateTime<Local>,

    on_char: Box<dyn Fn(&str) + Send + Sync>,
}

impl TextPattern {
    pub fn new(on_char: Box<dyn Fn(&str) + Send + Sync>) -> Self {
        Self {
            last_hand: [false; 5],

            current_value: None,
            current_mode: WritingMode::Letters,

            max_ms_delay: 1000,
            last_moved_time: Local::now(),

            on_char,
        }
    }

    pub fn set_max_ms_delay(&mut self, max_ms_delay: u32) {
        self.max_ms_delay = max_ms_delay;
    }

    fn apply_value(&mut self, value: u8) {
        if self.current_mode == WritingMode::Numbers {
            if value == 30 {
                (self.on_char)(&"0");
            } else {
                (self.on_char)(value.to_string().as_str());
            }

            self.current_mode = WritingMode::Letters;
        }

        match value {
            NUMBER_MODE_VALUE => self.current_mode = WritingMode::Numbers,
            BACKSPACE_VALUE => (self.on_char)(&"\x08"),
            DOT_VALUE => (self.on_char)(&"."),
            SPACE_VALUE => (self.on_char)(&" "),
            _ => (self.on_char)(ALPHABET[value as usize - 1]),
        }
    }

    pub fn process_moved_fingers(&mut self, moved_fingers: &[bool; 5], time: DateTime<Local>) {
        if self.last_hand == *moved_fingers {
            return;
        }
        self.last_hand = *moved_fingers;

        let new_hand_value = compute_hand_value(moved_fingers);

        match (new_hand_value, self.current_value) {
            (Some(new_hand_value), None) => {
                self.current_value = Some(new_hand_value);
                self.last_moved_time = time;
            }
            (Some(new_hand_value), Some(current_value)) => {
                let total_value = current_value * 5 + new_hand_value;

                self.apply_value(total_value);
                self.current_value = None;
                self.last_moved_time = time;
            }
            (None, Some(current_value)) => {
                let elapsed_time = time
                    .signed_duration_since(self.last_moved_time)
                    .num_milliseconds() as u32;

                if elapsed_time > self.max_ms_delay {
                    self.apply_value(current_value);
                    self.current_value = None;
                }
            }
            (None, None) => {}
        }
    }
}

pub fn compute_hand_value(moved_fingers: &[bool; 5]) -> Option<u8> {
    moved_fingers
        .iter()
        .enumerate()
        .find(|(_, is_pressed)| **is_pressed)
        .map(|(i, _)| (i + 1) as u8)
}
