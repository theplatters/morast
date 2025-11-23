#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CardID(u32);
impl CardID {
    // Existing methods
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl Into<CardID> for u16 {
    fn into(self) -> CardID {
        CardID::new(self as u32)
    }
}

impl Into<CardID> for u32 {
    fn into(self) -> CardID {
        CardID::new(self)
    }
}
