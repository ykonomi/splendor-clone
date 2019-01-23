use crate::card::Card;
use crate::color::Color;
use crate::level::Level;
use crate::noble_tile::NobleTile;
use crate::token_stack::Token;
use crate::token_stack::TokenStack;

use ndarray::Array2;
use std::fmt;

use std::collections::HashMap;

const LIMIT_OF_GETTING_SAME_TOKEN: u8 = 4;

const COORDINATE: [(u8, u8); 12] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (1, 0),
    (1, 1),
    (1, 2),
    (1, 3),
    (2, 0),
    (2, 1),
    (2, 2),
    (2, 3),
];

#[derive(Clone)]
pub struct Board {
    board: Array2<Card>,
    stack: HashMap<Level, Vec<Card>>,
    token_stack: TokenStack,
    noble_tile: Vec<NobleTile>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
貴族タイル: {:?}
----------------------------------------------------------------------------------------------
{}| {}| {}| {}
{}| {}| {}| {}
{}| {}| {}| {}
----------------------------------------------------------------------------------------------
レベル3 残り残数: {}
レベル2 残り残数: {}
レベル1 残り残数: {}
黒トークン 残り枚数: {}
白トークン 残り枚数: {}
赤トークン 残り枚数: {}
青トークン 残り枚数: {}
緑トークン 残り枚数: {}
金トークン 残り枚数: {}
        ",
            self.noble_tile,
            self.board[(0, 0)],
            self.board[(0, 1)],
            self.board[(0, 2)],
            self.board[(0, 3)],
            self.board[(1, 0)],
            self.board[(1, 1)],
            self.board[(1, 2)],
            self.board[(1, 3)],
            self.board[(2, 0)],
            self.board[(2, 1)],
            self.board[(2, 2)],
            self.board[(2, 3)],
            self.stack.get(&Level::One).unwrap().len(),
            self.stack.get(&Level::Two).unwrap().len(),
            self.stack.get(&Level::Three).unwrap().len(),
            self.token_stack.len(Color::Black),
            self.token_stack.len(Color::White),
            self.token_stack.len(Color::Red),
            self.token_stack.len(Color::Blue),
            self.token_stack.len(Color::Green),
            self.token_stack.len(Color::Gold)
        )
    }
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            board: Array2::<Card>::default((3, 4)),
            stack: HashMap::new(),
            token_stack: TokenStack::new(),
            noble_tile: vec![],
        };

        let cards = Card::load("json/card.json");

        let mut level1_stack = vec![];
        let mut level2_stack = vec![];
        let mut level3_stack = vec![];
        for card in cards.into_iter() {
            match card {
                Card { level: 1, .. } => level1_stack.push(card),
                Card { level: 2, .. } => level2_stack.push(card),
                Card { level: 3, .. } => level3_stack.push(card),
                Card { level: _, .. } => unreachable!(),
            }
        }

        // シャッフルする
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        level1_stack.shuffle(&mut rng);
        level2_stack.shuffle(&mut rng);
        level3_stack.shuffle(&mut rng);

        board.stack.insert(Level::One, level1_stack);
        board.stack.insert(Level::Two, level2_stack);
        board.stack.insert(Level::Three, level3_stack);

        for (x, y) in COORDINATE.iter() {
            board.refill(*x, *y);
        }

        use crate::color::Color::*;
        let colors = [Black, White, Red, Blue, Green, Gold];
        for color in colors.iter() {
            board.token_stack.addn(TokenStack::create_stack(*color));
        }

        board.noble_tile = NobleTile::create_stack();

        board
    }
    pub fn peek_card(&self, x: u8, y: u8) -> Option<&Card> {
        self.board.get((x as usize, y as usize))
    }
    pub fn get_card(&mut self, x: u8, y: u8) -> Option<Card> {
        match self.board.get_mut((x as usize, y as usize)) {
            Some(card) => {
                let card2 = card.clone();
                self.refill(x, y);
                Some(card2)
            }
            None => None,
        }
    }
    pub fn get_stack_card(&mut self, level: Level) -> Option<Card> {
        self.stack.get_mut(&level).unwrap().pop()
    }
    pub fn uget_card(&mut self, x: u8, y: u8) -> Card {
        let card = self.board.get_mut((x as usize, y as usize)).unwrap();
        let card2 = card.clone();
        self.refill(x, y);
        card2
    }
    pub fn get_token(&mut self, color: Color) -> Option<Token> {
        self.token_stack.remove(color)
    }
    pub fn uget_token(&mut self, color: Color) -> Token {
        self.token_stack.remove(color).unwrap()
    }
    pub fn get_token_stack(&mut self) -> &mut TokenStack {
        &mut self.token_stack
    }
    pub fn can_get_token(&self, color: Color) -> bool {
        self.get_number_of_tokens(color) >= LIMIT_OF_GETTING_SAME_TOKEN
    }
    pub fn get_noble_tile(&mut self) -> &mut Vec<NobleTile> {
        &mut self.noble_tile
    }
    fn get_number_of_tokens(&self, color: Color) -> u8 {
        self.token_stack.len(color)
    }
    fn refill(&mut self, x: u8, y: u8) {
        let stack = match x {
            0 => self.stack.get_mut(&Level::Three).unwrap(),
            1 => self.stack.get_mut(&Level::Two).unwrap(),
            2 => self.stack.get_mut(&Level::One).unwrap(),
            _ => unreachable!(),
        };

        match stack.pop() {
            Some(card) => self.board[[x as usize, y as usize]] = card,
            None => (),
        }
    }
}
