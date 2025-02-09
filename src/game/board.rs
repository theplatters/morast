/*
use std::collections::HashMap;

use macroquad::math::U16Vec2;

use super::events::{event::Event, event_handler::EventHandler};

#[derive(Debug)]
pub struct Tile {
    ontile: Vec<super::card::Card>,
}

#[derive(Debug)]
pub struct Board {
    tiles: HashMap<U16Vec2, Tile>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }
}

impl EventHandler for Board {
    fn handle_event(&mut self, event: &Event) -> Vec<Event> {
        match event {
            Event::DrawCard(_) => todo!(),
            Event::DiscardCard(_) => todo!(),
            Event::SendCardToHand(card_action) => todo!(),
            Event::SendCardToDiscard(card_action) => todo!(),
            Event::CardDrawn(_) => todo!(),
            Event::DeckEmpty(_) => todo!(),
            Event::CardDiscarded(_) => todo!(),
            Event::HandEmpty(_) => todo!(),
            _ => todo!(),
        };
    }
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
