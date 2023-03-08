use log::info;

use crate::interface::engine;

#[macro_use]
extern crate crossterm;

mod architecture;
mod parser;
mod commands;
mod helpers;
mod interface;

fn main() {

    log4rs::init_file("log_config.yaml", Default::default()).unwrap();

    info!("Rush Initiated");

    engine::run();
}
