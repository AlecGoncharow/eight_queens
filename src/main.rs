use rand::prelude::*;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

struct GameState {
    board: Board,
}

impl GameState {
    fn new() -> Self {
        Self {
            board: Board::new(8),
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}


struct Board {
    size: usize,
    blocks: Vec<Block>,
    conflicts: i32,
}

impl Board {
    fn new(size: usize) -> Self {
        Self {
            size,
            blocks: vec![Block::default(); size*size],
            conflicts: 0
        }
    }

    fn new_from_queens(queens: Vec<usize>) -> Self {
        let size = queens.len();
        let mut blocks = vec![Block::default(); size*size];
        for queen in queens {
            blocks[queen] = Block::queen();
        }
        Self {
            size,
            blocks,
            conflicts: 0,
        }
    }

    fn initialize_queens(&mut self) {
        let mut rng = rand::thread_rng();
        for row in 0..self.size {
            let col = rng.gen_range(0, self.size);
            self[row * self.size + col] = Block::queen();
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(8)
    }
}

struct Block {
    state: BlockState,
    attacked: bool,
}

impl Block {
   fn queen() -> Self {
        Self {
            state: BlockState::Queen,
            attacked: false,
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            state: BlockState::Empty,
            attacked: false
        }
    }
}

enum BlockState {
    Empty,
    Queen,
    AttackedQueen,
}


fn main() {
    let ctx = &mut ContextBuilder::new("8 queens", "Gunthorian")
        .build()
        .expect("context builder failed");


    println!("Hello, world!");
}
