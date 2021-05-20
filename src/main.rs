#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rocket::http::RawStr;
use std::{mem, env};
use rocket::response::content::Json;

mod game;

fn main() {

    //Start the game!
    game::start();
}