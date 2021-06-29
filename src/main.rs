mod cpu;
mod dashboard;
mod fmt;
mod gpu;
mod header;
mod style;
mod system;

extern crate log;
extern crate simplelog;

use crate::dashboard::Dashboard;
use log::LevelFilter;
use simplelog::{ConfigBuilder, WriteLogger};
use std::fs::OpenOptions;

fn main() {
    init_logger();

    let mut dash = get_dash();

    dash.run();

    dash.destroy();
}

fn init_logger() {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("sys-dash.log")
        .expect("Unable to open log file.");
    let config = ConfigBuilder::new().set_time_to_local(true).build();

    WriteLogger::init(LevelFilter::Debug, config, file).expect("Unable to initialize logger.");
}

fn get_dash() -> Box<Dashboard> {
    Box::new(Dashboard::new())
}
