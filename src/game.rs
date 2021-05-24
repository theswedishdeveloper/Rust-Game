/*
 *
 * Copyright 2020 Benjamin Ojanne - All rights reserved
 *
*/
use std::ptr::null;
use std::time::{Duration, Instant};

use cgmath::mint::Vector2;
use cgmath::num_traits::Pow;
use ggez::{Context, ContextBuilder, GameResult, graphics};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{Color, draw, Drawable, DrawParam, Image, mint, Rect};
use ggez::graphics::spritebatch::{SpriteBatch, SpriteIdx};
use ggez::input::mouse::MouseButton;
use rand::Rng;
use std::alloc::handle_alloc_error;

const AIR_SPEED: f32 = 8.0;
const ENEMY_MAX_AIR_SPEED: i16 = 20;
const MAX_ENEMIES: i16 = 5;

static mut RIGHT_KEY_PRESSED: bool = false;
static mut LEFT_KEY_PRESSED: bool = false;
static mut DOWN_KEY_PRESSED: bool = false;
static mut UP_KEY_PRESSED: bool = false;

const GRID_SIZE: (i16, i16) = (35, 20);
const GRID_CELL_SIZE: (i16, i16) = (34, 34);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const UPDATES_PER_SECOND: f32 = 120.0;

// And we get the milliseconds of delay that this update rate corresponds to.
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

const ENEMIES: Vec<Enemy> = Vec::new();

/* Start Function */
pub fn start() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_first_game", "Benjamin Ojanne")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("AIRPLANE GAME | Version 1.0.0")
                .icon("/plane.png"),
        )
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("Failed to create ggez context!!! Try again");

    let game_state = MyGameState::new(&mut ctx);

    //Run the game!
    event::run(ctx, event_loop, game_state);
}

fn get_random(min_value: i16, max_value: i16) -> i16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max_value)
}

struct MyGameState {
    plane: Plane,
    background: Background,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    game_over: bool,
    last_update: Instant,
}

impl MyGameState {
    pub fn new(ctx: &mut Context) -> Self {
        // Load / create resources.

        let mut image = graphics::Image::new(ctx, "/plane.png").unwrap();
        image.set_filter(graphics::FilterMode::Nearest);

        let mut background_img = graphics::Image::new(ctx, "/background.png").unwrap();
        background_img.set_filter(graphics::FilterMode::Nearest);

        let img_width = background_img.width();

        let mut bullets: Vec<Bullet> = Vec::new();

        //10 - times
        for _ in 1..11 {
            let mut shot_img = graphics::Image::new(ctx, "/fire_ball_1.png").unwrap();
            bullets.push(Bullet::new(shot_img, false));
        }

        //10 - times
        for _ in 1..11 {
            let mut shot_img = graphics::Image::new(ctx, "/torpedo.png").unwrap();
            bullets.push(Bullet::new(shot_img, true));
        }

        MyGameState {
            plane: Plane::new(image),
            background: Background::new(background_img, img_width),
            enemies: ENEMIES,
            bullets,
            game_over: false,
            last_update: Instant::now(),
        }
    }
}

pub fn should_spawn_enemy() -> bool {
    (ENEMIES.len() as i16) < MAX_ENEMIES
}

pub fn spawn_enemy(ctx: &mut Context) {
    let mut enemy_img = graphics::Image::new(ctx, "/spacecraft.png").unwrap();
    enemy_img.set_filter(graphics::FilterMode::Nearest);

    let x: f32 = rand::thread_rng().gen_range((SCREEN_SIZE.0 / 2_f32)..SCREEN_SIZE.0);
    let y: f32 = rand::thread_rng().gen_range((SCREEN_SIZE.1 / 2_f32)..SCREEN_SIZE.1);
    let speed: i16 = get_random(2, ENEMY_MAX_AIR_SPEED);

    let enemy: Enemy = Enemy::new(enemy_img, x, y, speed);
    ENEMIES.push(enemy);
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

    fn update(&mut self) {
        self.x = self.x - 3.0;
        self.x2 = self.x2 - 3.0;
        if self.x2 < 0.0 {
            self.x2 = SCREEN_SIZE.0;
            self.x = 0.0;
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut params = graphics::DrawParam::new();

        params = params.scale(mint::Vector2 {
            x: SCREEN_SIZE.0 / self.image.width() as f32,
            y: SCREEN_SIZE.1 / self.image.height() as f32,
        });

        params = params.dest(mint::Vector2 { x: self.x, y: 0.0 });

        graphics::draw(ctx, &self.image, params)?;

        params = params.dest(mint::Vector2 { x: self.x2, y: 0.0 });

        graphics::draw(ctx, &self.image, params)?;

        //Garbage collection
        std::mem::drop(params);

        Ok(())
    }
}

struct Enemy {
    image: graphics::Image,
    x: f32,
    y: f32,
    speed: i16,
}

impl Enemy {
    pub fn new(image: graphics::Image, spawn_x: f32, spawn_y: f32, speed: i16) -> Self {
        Enemy {
            image,
            x: spawn_x,
            y: spawn_y,
            speed,
        }
    }

    fn update(&mut self) {}

    fn draw(&mut self, ctx: Context) {}
}

struct Plane {
    image: graphics::Image,
    boost: bool,
    pos: nalgebra::Point2<f32>,
    x: f32,
    y: f32,
    angle: f32,
    speed: f32,
}

impl Plane {
    pub fn new(image: graphics::Image) -> Self {
        Plane {
            image,
            boost: false,
            pos: nalgebra::Point2::new(0.0, 0.0),
            x: 400.0,
            y: 400.0,
            angle: 0.0,
            speed: 0.0,
        }
    }

    pub unsafe fn update(&mut self) {
        if RIGHT_KEY_PRESSED && self.speed < AIR_SPEED {
            self.speed = AIR_SPEED;
        } else if LEFT_KEY_PRESSED && self.speed > -AIR_SPEED {
            self.speed = -AIR_SPEED;
        }

        if UP_KEY_PRESSED {
            self.y -= AIR_SPEED / 2.0;
        }

        if DOWN_KEY_PRESSED {
            self.y += AIR_SPEED / 2.0;
        }

        if RIGHT_KEY_PRESSED && !LEFT_KEY_PRESSED {
            self.x += self.speed;
        } else if LEFT_KEY_PRESSED && !RIGHT_KEY_PRESSED {
            self.x += self.speed;
        }

        if self.x > SCREEN_SIZE.0 + (self.image.width() as f32) {
            self.x = 0.0;
        } else if self.x < -(self.image.width() as f32) {
            self.x = SCREEN_SIZE.0;
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut params = graphics::DrawParam::new();

        params = params.scale(mint::Vector2 { x: 0.125, y: 0.125 });

        params = params.dest(mint::Vector2 {
            x: self.x,
            y: self.y,
        });

        params = params.rotation(self.angle);

        params = params.offset(mint::Point2 { x: 0.5, y: 0.5 });

        graphics::draw(ctx, &self.image, params)?;

        std::mem::drop(params);

        Ok(())
    }
}

struct Bullet {
    img: Image,
    x: f32,
    y: f32,
    active: bool,
    torpedo: bool,
}

impl Bullet {
    fn new(img: Image, torpedo: bool) -> Self {
        Bullet {
            img,
            x: 0.0,
            y: 0.0,
            active: false,
            torpedo,
        }
    }
}

struct Explosion {
    sprite_batch: SpriteBatch,
}

impl Explosion {
    fn new(sprite_batch: SpriteBatch) -> Self {
        Explosion { sprite_batch }
    }
}

impl EventHandler for MyGameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if !(Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE)) {
            return Ok(());
        }
        if !self.game_over {
            unsafe {
                self.plane.update();
            }
            self.background.update();

            //self::handle_enemies(_ctx);
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

        //draw bullets
        for bullet in self.bullets.iter_mut() {
            if bullet.active {
                bullet.x = bullet.x + 15.0;

                if bullet.x > SCREEN_SIZE.0 {
                    bullet.active = false;
                }

                let mut params: DrawParam = DrawParam::new();

                params = params.dest(mint::Vector2 {
                    x: bullet.x,
                    y: bullet.y,
                });

                params = params.offset(mint::Point2 { x: 0.0, y: 0.5 });

                params = params.scale(mint::Vector2 {
                    x: if bullet.torpedo { 0.25 } else { 0.5 },
                    y: if bullet.torpedo { 0.25 } else { 0.5 },
                });

                graphics::draw(ctx, &bullet.img, params)?;
            }
        }

        //Display new frame!
        graphics::present(ctx)?;
        //Yield the current thread until the next update
        ggez::timer::yield_now();
        //Return success.
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _key_mod: KeyMods,
        _repeat: bool,
    ) {
        unsafe {
            if _keycode == KeyCode::Right {
                RIGHT_KEY_PRESSED = true;
            } else if _keycode == KeyCode::Left {
                LEFT_KEY_PRESSED = true;
            } else if _keycode == KeyCode::Up {
                UP_KEY_PRESSED = true;
            } else if _keycode == KeyCode::Down {
                DOWN_KEY_PRESSED = true;
            } else if _keycode == KeyCode::Space {
                self.plane.boost = true;
            }
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {
        unsafe {
            if _keycode == KeyCode::F {
                for bullet in self.bullets.iter_mut() {
                    if !bullet.active && !bullet.torpedo {
                        bullet.y = self.plane.y;
                        bullet.x = self.plane.x;
                        bullet.active = true;
                        break;
                    }
                }
            } else if _keycode == KeyCode::Space {
                for bullet in self.bullets.iter_mut() {
                    if !bullet.active && bullet.torpedo {
                        bullet.y = self.plane.y;
                        bullet.x = self.plane.x;
                        bullet.active = true;
                        break;
                    }
                }
            } else if _keycode == KeyCode::Right {
                RIGHT_KEY_PRESSED = false;
            } else if _keycode == KeyCode::Left {
                LEFT_KEY_PRESSED = false;
            } else if _keycode == KeyCode::Up {
                UP_KEY_PRESSED = false;
            } else if _keycode == KeyCode::Down {
                DOWN_KEY_PRESSED = false;
            } else if _keycode == KeyCode::Space {
                self.plane.boost = false;
            }
        }
    }
}

//Update enemies
pub fn handle_enemies(ctx: &mut Context) {
    let spawn_enemies = should_spawn_enemy();

    if spawn_enemies {
        spawn_enemy(ctx);
    }

    for enemy in ENEMIES {}
}
