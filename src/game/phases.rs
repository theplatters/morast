#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    Start, // Beginning of a turn
    Main,  // During the turn
    End,   // End of a turn
}
