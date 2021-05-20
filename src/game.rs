use std::time::Instant;

use ggez::{Context, ContextBuilder, GameResult, graphics};
use ggez::event::{self, EventHandler};

//Define the size of our game board
const GRID_SIZE: (i16, i16) = (30, 20);
//Define the pixel size of each tile, which we make 32x32 pixels.
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

pub fn start() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_first_game", "Benjamin Ojanne")
        .window_setup(ggez::conf::WindowSetup::default().title("Monster Game - Version: 1.0.0"))
        .build()
        .expect("Failed to create ggez context!!");

    //Create an instance
    let game_state = MyGameState::new(&mut ctx);

    //Run the game!
    event::run(ctx, event_loop, my_game);
}

struct MyGameState {
    player: Player,
    enemy: Enemy,
    game_over: bool,
    last_update: Instant,
}

impl MyGameState {
    pub fn new() -> Self {
        // Load/create resources.

        let player_pos = (GRID_SIZE.0 / 4, GRID_SIZE.1 / 2).into();

        MyGameState {}
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //clear screen
        graphics::clear(ctx, graphics::Color::WHITE);

        //show the graphics!
        graphics::present(ctx)
    }
}

