use chrono::{DateTime, Local};

pub const FINGERS_ORDER: [u8; 5] = [0, 1, 2, 3, 4];

pub const DEFAULT_PATTERN_MAX_DELAY: u32 = 500;
pub const DEFAULT_REPEATING_PATTERN_DELAY: u32 = 1000;

pub struct ReapeatingPattern {
    pattern: Pattern,

    pub nb_done: u32,

    max_ms_delay: u32,
    last_time_done: DateTime<Local>,
}

impl ReapeatingPattern {
    pub fn new(pattern: Pattern, max_ms_delay: u32) -> Self {
        Self {
            pattern,
            nb_done: 0,

            max_ms_delay,
            last_time_done: chrono::Local::now(),
        }
    }

    pub fn process_moved_fingers(&mut self, moved_fingers: &[bool; 5], time: DateTime<Local>) {
        let elapsed_time = time
            .signed_duration_since(self.last_time_done)
            .num_milliseconds() as u32;
        if elapsed_time > self.max_ms_delay {
            self.nb_done = 0;
        }

        self.pattern.process_moved_fingers(moved_fingers, time);

        if self.pattern.is_done() {
            self.nb_done += 1;
            self.last_time_done = time;
            self.pattern.reset();
        }
    }
}

pub struct Pattern {
    fingers_order: Vec<u8>,
    current_index: usize,

    max_ms_delay: u32,
    last_finger_time: DateTime<Local>,
}

impl Pattern {
    pub fn new(fingers_order: Vec<u8>, max_ms_delay: u32) -> Self {
        Self {
            fingers_order,
            max_ms_delay,

            current_index: 0,
            last_finger_time: chrono::Local::now(),
        }
    }

    pub fn process_moved_fingers(&mut self, moved_fingers: &[bool; 5], time: DateTime<Local>) {
        let elapsed_time = time
            .signed_duration_since(self.last_finger_time)
            .num_milliseconds() as u32;
        if elapsed_time > self.max_ms_delay {
            self.current_index = 0;
        }

        if !moved_fingers[self.fingers_order[self.current_index] as usize] {
            return;
        }

        if moved_fingers.iter().filter(|v| **v).count() > 1 {
            return;
        }

        self.current_index += 1;
        self.last_finger_time = time;
    }

    pub fn is_done(&self) -> bool {
        self.current_index == self.fingers_order.len()
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}
