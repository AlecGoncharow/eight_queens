use ggez::{Context, ContextBuilder, GameResult, graphics};
use rand::prelude::*;


// Here we define the size of our game board in terms of how many grid
// cells it will take up. We choose to make a 8x8 game board.
pub const GRID_SIZE: (i16, i16) = (8, 8);
// Now we define the pixel size of each tile, which we make 64x64 pixels.
pub const GRID_CELL_SIZE: (i16, i16) = (64, 64);

// Next we define how large we want our actual window to be by multiplying
// the components of our grid size by its corresponding pixel size.
pub const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const ODD_COLOR: (u8, u8, u8) = (202, 164, 114);
const EVEN_COLOR: (u8, u8, u8) = (184, 134, 69);

pub struct Board {
    pub size: usize,
    pub blocks: Blocks,
    pub conflicts: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}
impl From<(usize, usize)> for GridPosition {
    fn from(pos: (usize, usize)) -> Self {
        GridPosition { x: pos.0 as i16, y: pos.1 as i16}
    }
}
impl From<(i32, i32)> for GridPosition {
    fn from(pos: (i32, i32)) -> Self {
        GridPosition { x: pos.0 as i16, y: pos.1 as i16}
    }
}
impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            blocks: Blocks::new(size),
            conflicts: 0
        }
    }

    pub fn new_from_queens(queens: Vec<usize>) -> Self {
        Self {
            size: queens.len(),
            blocks: Blocks::new_from_queens(queens),
            conflicts: 0,
        }
    }

    pub fn initialize_queens(&mut self) {
        self.blocks.initialize_queens(self.size);
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.blocks.draw(ctx)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(8)
    }
}

pub struct Blocks {
    pub blocks: Vec<Block>
}

impl Blocks {
    pub fn new(size: usize) -> Self {
        Self {
            blocks: Self::make_checkerboard(size),
        }
    }

    pub fn new_from_queens(queens: Vec<usize>) -> Self {
        let size = queens.len();
        let mut blocks = vec![Block::default(); size*size];
        for queen in queens {
            blocks[queen] = Block::queen();
        }

        Self {
            blocks
        }
    }

    pub fn initialize_queens(&mut self, size: usize) {
        let mut rng = rand::thread_rng();
        for row in 0..size {
            let col = rng.gen_range(0, size);
            self.blocks[row * size + col] = Block::queen();
        }
    }

    pub fn make_checkerboard(size: usize) -> Vec<Block> {
        let mut blocks = vec![Block::default(); size*size];
        for i in 0..size {
            for j in 0..size {
                blocks[i * size + j].pos((j, i).into());
            }
        }
        blocks
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        for block in self.blocks.iter() {
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                block.pos.into(),
                block.color
            )?;
            graphics::draw(ctx, &rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Block {
    pub state: BlockState,
    pub attacked: bool,
    pub pos: GridPosition,
    pub color: graphics::Color
}

impl Block {
    pub fn queen() -> Self {
        Self {
            state: BlockState::Queen,
            attacked: false,
            pos: (0, 0).into(),
            color: ODD_COLOR.into(),
        }
    }

    //update block color with position to create checkerboard effect
    pub fn pos(&mut self, pos: GridPosition) {
        if pos.y % 2 == 0 {
            if pos.x % 2 == 0 {
                self.color = EVEN_COLOR.into();
            } else {
                self.color = ODD_COLOR.into();
            }
        } else {
            if pos.x % 2 == 0 {
                self.color = ODD_COLOR.into();
            } else {
                self.color = EVEN_COLOR.into();
            }
        }

        self.pos = pos;
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            state: BlockState::Empty,
            attacked: false,
            pos: (0, 0).into(),
            color: ODD_COLOR.into(),
        }
    }
}

#[derive(Clone)]
pub enum BlockState {
    Empty,
    Queen,
}


