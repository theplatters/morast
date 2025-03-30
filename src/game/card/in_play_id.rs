#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InPlayID(u32);
impl InPlayID {
    // Existing methods
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    // New next method with overflow protection
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<InPlayID> for i32 {
    fn from(value: InPlayID) -> Self {
        value.0 as i32
    }
}

impl From<i64> for InPlayID {
    fn from(value: i64) -> Self {
        InPlayID(value as u32)
    }
}

impl From<InPlayID> for i64 {
    fn from(value: InPlayID) -> Self {
        value.0 as i64
    }
}
