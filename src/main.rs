mod app;
mod common;

use app::MyApp;
use chrono::Local;
use common::run_app;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::{error::Error, io::Write};

fn init_logger(target: Target) {
    Builder::from_default_env()
        .target(target)
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    init_logger(Target::Stdout);

    #[cfg(profile)]
    Client::start();
    run_app::<MyApp>()?;
    Ok(())
}
