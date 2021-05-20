/*
 Copyright 2020 Benjamin Ojanne
 */
use std::time::{Duration, Instant};

use ggez::{Context, ContextBuilder, GameResult, graphics};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::Rect;
use rand::Rng;

//Define the size of our game board
const GRID_SIZE: (i16, i16) = (30, 20);
//Define the pixel size of each tile, which we make 32x32 pixels.
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

// Here we're defining how often we want our game to update. This will be
// important later so that we don't have our snake fly across the screen because
// it's moving a full tile every frame.
const UPDATES_PER_SECOND: f32 = 8.0;
// And we get the milliseconds of delay that this update rate corresponds to.
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

/* Start main function */
pub fn start() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_first_game", "Benjamin Ojanne")
        .window_setup(ggez::conf::WindowSetup::default().title("Monster Game - Version: 1.0.0"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("Failed to create ggez context!!");

    //Create an instance
    let game_state = MyGameState::new();

    //Run the game!
    event::run(ctx, event_loop, game_state);
}

fn get_random(max_value: i16) -> i16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max_value)
}

struct MyGameState {
    player: Player,
    //enemy: Enemy,
    game_over: bool,
    last_update: Instant,
}

impl MyGameState {
    pub fn new() -> Self {
        // Load/create resources.

        let player_pos = (GRID_SIZE.0 / 2, GRID_SIZE.1 / 2).into();

        MyGameState {
            player: Player::new(player_pos),
            game_over: false,
            last_update: Instant::now(),
        }
    }
}

struct Player {
    position: MapPosition,
}

impl Player {
    pub fn new(position: MapPosition) -> Self {
        Player {
            position
        }
    }

    //&mut self, food: &Food
    pub fn update(&mut self) {}

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // First we set the color to draw with, in this case all food will be
        // colored blue.
        // let color = [0.0, 0.0, 1.0, 1.0].into();
        // Then we draw a rectangle with the Fill draw mode, and we convert the
        // Food's position into a `ggez::Rect` using `.into()` which we can do
        // since we implemented `From<GridPosition>` for `Rect` earlier.

        // let color = graphics::Color::WHITE;

        // And then we do the same for the head, instead making it fully red to distinguish it.
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, 10.0, 10.0),
            [1.0, 0.5, 0.0, 1.0].into(),
        )?;

        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 }, ))?;

        Ok(())
    }
}

struct MapPosition {
    x: i16,
    y: i16,
}

impl MapPosition {
    pub fn new(x: i16, y: i16) -> Self {
        MapPosition {
            x,
            y,
        }
    }

    fn get_random_pos() -> Self {
        (get_random(SCREEN_SIZE.0 as i16), get_random(SCREEN_SIZE.1 as i16)).into()
    }
}

impl From<(i16, i16)> for MapPosition {
    fn from(pos: (i16, i16)) -> Self {
        MapPosition { x: pos.0, y: pos.1 }
    }
}

impl From<MapPosition> for graphics::Rect {
    fn from(pos: MapPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl EventHandler for MyGameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if !(Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE)) {
            return Ok(());
        }

        if !self.game_over {
            self.player.update();
        }

        // If we updated, we set our last_update to be now
        self.last_update = Instant::now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // First we clear the screen to a nice (well, maybe pretty glaring ;)) green
        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());
        // Then we tell the snake and the food to draw themselves
        self.player.draw(ctx);
        // Finally we call graphics::present to cycle the gpu's framebuffer and display
        // the new frame we just drew.
        graphics::present(ctx);
        // We yield the current thread until the next update
        ggez::timer::yield_now();
        // And return success.
        Ok(())
    }

    /// key_down_event gets fired when a key gets pressed.
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Down => {
                println!("Pressed down key!");
            }
            _ => { println!("Unknown key pressed!"); }
        }
    }
}

