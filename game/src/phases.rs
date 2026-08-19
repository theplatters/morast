use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Phase {
    Start, // Beginning of a turn
    Main,  // During the turn
    End,   // End of a turn
}

impl From<i32> for Phase {
    fn from(value: i32) -> Self {
        match value {
            0 => Phase::Start,
            1 => Phase::Main,
            2 => Phase::End,
            _ => Phase::End,
        }
    }
}
