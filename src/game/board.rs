/*
use std::collections::HashMap;

struct Index {
    x: usize,
    y: usize,
}

pub struct Tile {}

pub struct Board {
    tiles: HashMap<Index, Tile>,
}
*/

#[derive(Clone,Copy,Debug)]

enum Tile{
    Empty,
    Character(char), //character on a tile
}

impl Tile{
    fn display(&self) -> char{
        match self{
            Tile::Empty => '□', //empty tile
            Tile::Character(c) => *c, //character tile
        }
    }
}

fn print_board(board: &Vec<Vec<Tile>>){
    for row in board{
        for tile in row{
            print!("{}", tile.display());
        }
        println!();
    }
}

fn main(){
    let mut board: Vec<Vec<Tile>> = vec![vec![Tile::Empty; 10]; 10];
    
    //for demonstration: place character at spec. location
    //board[2][3] = Tile::Character('A');
    //board[5][5] = Tile::Character('B');

    print_board(&board);
}
