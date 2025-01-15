use std::collections::HashMap;

struct Index {
    x: usize,
    y: usize,
}

pub struct Tile {}

pub struct Board {
    tiles: HashMap<Index, Tile>,
}
