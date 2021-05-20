/*
  Copyright 2020 Benjamin Ojanne - All rights reserved
 */
use std::ptr::null;
use std::time::{Duration, Instant};

use cgmath::mint::Vector2;
use cgmath::num_traits::Pow;
use ggez::{Context, ContextBuilder, GameResult, graphics};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{Color, Image, mint, Rect};
use ggez::input::mouse::MouseButton;
use rand::Rng;

const MAX_AIR_SPEED: f32 = 15.0;
const MIN_AIR_SPEED: f32 = 0.0;

//const GRAVITY: i16 = 3;

//Screen size
const GRID_SIZE: (i16, i16) = (35, 20);
const GRID_CELL_SIZE: (i16, i16) = (34, 34);
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const UPDATES_PER_SECOND: f32 = 50.0;

// And we get the milliseconds of delay that this update rate corresponds to.
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

/* Start Function */
pub fn start() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_first_game", "Benjamin Ojanne")
        .window_setup(ggez::conf::WindowSetup::default().title("Airplane Game - Version: 1.0.0").icon("/plane.png"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("Failed to create ggez context!!");

    //Create an instance
    let game_state = MyGameState::new(&mut ctx);

    //Run the game!
    event::run(ctx, event_loop, game_state);
}

/*fn get_random(max_value: i16) -> i16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max_value)
}*/

struct MyGameState {
    plane: Plane,
    background: Background,
    game_over: bool,
    last_update: Instant,
}

impl MyGameState {
    pub fn new(ctx: &mut Context) -> Self {

        // Load / create resources.

        let player_pos = (GRID_SIZE.0 / 2, GRID_SIZE.1 / 2).into();

        let mut image = graphics::Image::new(ctx, "/plane.png").unwrap();
        image.set_filter(graphics::FilterMode::Nearest);

        let mut background_img = graphics::Image::new(ctx, "/background.png").unwrap();
        background_img.set_filter(graphics::FilterMode::Nearest);

        let img_width = background_img.width();

        MyGameState {
            plane: Plane::new(player_pos, image),
            background: Background::new(background_img, img_width),
            game_over: false,
            last_update: Instant::now(),
        }
    }
}

struct Background {
    x: f32,
    x2: f32,
    y: i16,
    image: Image,
    width: u16,
}

impl Background {
    pub fn new(image: graphics::Image, img_width: u16) -> Self {
        Background {
            x: 0.0,
            x2: 0.0,
            y: 0,
            image,
            width: img_width,
        }
    }

    fn update(&mut self, speed: f32) {
        self.x = self.x - 1.0;
        self.x2 = self.x2 - 1.0;
        if self.x2 < 0.0 {
            self.x2 = SCREEN_SIZE.0;
            self.x = 0.0;
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut params = graphics::DrawParam::new();

        //  let dif = (SCREEN_SIZE.0 / (self.image.width() as f32)) * 1.2 - (SCREEN_SIZE.0 / (self.image.width() as f32));

        params = params.scale(mint::Vector2 {
            x: SCREEN_SIZE.0 / self.image.width() as f32,
            y: SCREEN_SIZE.1 / self.image.height() as f32,
        });

        params = params.dest(mint::Vector2 {
            x: self.x,
            y: 0.0,
        });

        graphics::draw(ctx, &self.image, params)?;

        params = params.dest(mint::Vector2 {
            x: self.x2,
            y: 0.0,
        });

        graphics::draw(ctx, &self.image, params)?;

        //Drop memory!
        std::mem::drop(params);

        Ok(())
    }
}

struct Plane {
    position: MapPosition,
    image: graphics::Image,
    accelerate: bool,
    flip: bool,
    rotate_right: bool,
    rotate_left: bool,
    x: f32,
    y: f32,
    speed: f32,
    rotation: f32,
}

impl Plane {
    pub fn new(position: MapPosition, image: graphics::Image) -> Self {
        Plane {
            position,
            image,
            accelerate: false,
            flip: false,
            rotate_right: false,
            rotate_left: false,
            x: 400.0,
            y: 400.0,
            speed: 0.0,
            rotation: 0.0,
        }
    }

    pub fn update(&mut self) {
        if self.accelerate && self.speed < MAX_AIR_SPEED {
            self.speed += 2.0;
        } else if !self.accelerate && self.speed > 0.0 {
            self.speed -= 2.0;
        }
        if self.rotate_right {
            self.rotation += 0.1;
        } else if self.rotate_left {
            self.rotation -= 0.1;
        }

        self.x = self.x + self.speed;

        if self.x > SCREEN_SIZE.0 + (self.image.width() as f32) {
            self.x = 0.0;
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        /* if self.move_right {
             if (self.position.x + &PLAYER_SPEED + self.width) <= SCREEN_SIZE.0 as i16 {
                 self.position.x += PLAYER_SPEED;
             }
         }

         if self.move_left {
             if (self.position.x - &PLAYER_SPEED) >= 0 {
                 self.position.x -= PLAYER_SPEED;
             }
         }*/


        // And then we do the same for the head, instead making it fully red to distinguish it.
        /*let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(self.position.x as f32, self.position.y as f32, self.width as f32, self.height as f32),
            [1.0, 0.5, 0.0, 1.0].into(),
        )?;

        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 }, ))?;*/

        let mut params = graphics::DrawParam::new();

        params = params.scale(mint::Vector2 { x: if self.flip { -0.15 } else { 0.15 }, y: 0.15 });

        params = params.dest(mint::Vector2 {
            x: self.x,
            y: self.y,
        });

        params = params.rotation(self.rotation);

        params = params.offset(mint::Point2 { x: 0.5, y: 0.5 });

        graphics::draw(ctx, &self.image, params)?;

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

    /*fn get_random_pos() -> Self {
        (get_random(SCREEN_SIZE.0 as i16), get_random(SCREEN_SIZE.1 as i16)).into()
    }*/
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
            GRID_CELL_SIZE.1 as i32, )
    }
}

impl EventHandler for MyGameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if !(Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE)) {
            return Ok(());
        }
        if !self.game_over {
            self.plane.update();
            self.background.update(self.plane.speed);
        }
        //Set last_update to be now
        self.last_update = Instant::now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK); //Cyan color!
        //Draw background first!
        self.background.draw(ctx)?;
        //draw player
        self.plane.draw(ctx)?;
        //Display new frame!
        graphics::present(ctx)?;
        //Yield the current thread until the next update
        ggez::timer::yield_now();
        //Return success.
        Ok(())
    }

    /*fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        if _button == MouseButton::Left {
            self.plane.accelerate = true;
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        if _button == MouseButton::Left {
            self.plane.accelerate = false;
        }
    }*/

    /*fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        let mut angle: f32 = 0.0;
        if self.plane.y < _y && self.plane.x < _x {
            let dif_x = f32::abs(self.plane.x - _x);
            let dif_y = f32::abs(self.plane.y - _y);
            let c = f32::sqrt((dif_x.pow(2) + dif_y.pow(2)));
            angle = f32::asin(f32::sin(90.0) * dif_y / c);
        } else if self.plane.y > _y && self.plane.x < _x {
            let dif_x = self.plane.x - _x;
            let dif_y = self.plane.y - _y;
            let c = f32::sqrt((dif_x.pow(2) + dif_y.pow(2)));
            angle = -f32::asin(f32::sin(90.0) * dif_y / c);
        } else if self.plane.y > _y && _x < self.plane.x {
            let dif_x = f32::abs(self.plane.x - _x);
            let dif_y = f32::abs(self.plane.y - _y);
            let c = f32::sqrt((dif_x.pow(2) + dif_y.pow(2)));
            angle = f32::asin(f32::sin(90.0) * dif_y / c);
        } else if self.plane.y < _y && _x < self.plane.x {
            let dif_x = self.plane.x - _x;
            let dif_y = self.plane.y - _y;
            let c = f32::sqrt((dif_x.pow(2) + dif_y.pow(2)));
            angle = f32::asin(f32::sin(90.0) * dif_y / c);
        }
        self.plane.rotation = angle;
        self.plane.flip = _x < self.plane.x;
    }*/

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        if keycode == KeyCode::Right {
            self.plane.rotate_right = true;
        }
        if keycode == KeyCode::Left {
            self.plane.rotate_left = true;
        }
        if keycode == KeyCode::Space {
            self.plane.accelerate = true;
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {
        if _keycode == KeyCode::Right {
            self.plane.rotate_right = false;
        }
        if _keycode == KeyCode::Left {
            self.plane.rotate_left = false;
        }
        if _keycode == KeyCode::Space {
            self.plane.accelerate = false;
        }
    }
}

