use log::{info, LevelFilter};
use log4rs::{append::file::FileAppender, encode::pattern::PatternEncoder, Config, config::{Appender, Root}};

use crate::interface::engine;

#[macro_use]
extern crate crossterm;

extern crate proc_macro;

mod architecture;
mod parser;
mod commands;
mod helpers;
mod interface;

fn main() {

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("log/rush.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(Root::builder().appender("file").build(LevelFilter::Debug))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();

    info!("Rush Initiated");

    engine::run();
}
