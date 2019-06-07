use rand::prelude::*;
use ggez::{Context, ContextBuilder, GameResult, graphics};
use ggez::event::{self, EventHandler};

mod board;
use board::Board;



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

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.board.draw(ctx);
        // Finally we call graphics::present to cycle the gpu's framebuffer and display
        // the new frame we just drew.
        graphics::present(ctx)?;
        // We yield the current thread until the next update
        ggez::timer::yield_now();
        // And return success.`
        Ok(())
    }
}



fn main() -> GameResult {
    let (ctx, events_loop) = &mut ContextBuilder::new("8 queens", "Gunthorian")
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("Snake!"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(ggez::conf::WindowMode::default().dimensions(board::SCREEN_SIZE.0, board::SCREEN_SIZE.1))
        .build()
        .expect("context builder failed");

    // Next we create a new instance of our GameState struct, which implements EventHandler
    let state = &mut GameState::new();
    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}
