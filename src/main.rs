#![feature(proc_macro_hygiene, decl_macro)]

use std::{env, mem};

use rocket::*;
use rocket::http::RawStr;
use rocket::response::content::Json;

mod game;

fn main() {
    //Start the game!
    game::start();
}