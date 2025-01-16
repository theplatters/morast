use std::collections::HashMap;

struct Index {
    x: usize,
    y: usize,
}

pub struct Tile {
    ontile: Vec<super::card::Card>,
}

pub struct Board {
    tiles: HashMap<Index, Tile>,
}
