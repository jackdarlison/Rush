use crate::interface::engine;

#[macro_use]
extern crate crossterm;

mod architecture;
mod parser;
mod commands;
mod helpers;
mod interface;

fn main() {
    println!("Welcome to Rush!");

    engine::run();
}
