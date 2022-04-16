#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;

use std::fs;

const VERSION: &str = "0.4.3";

mod cli;
mod configuration;
mod errors;

use crate::errors::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let builder = cli::get_builder();
    let builder = builder.process_file()?;
    let config = builder.build();
    simple_logger::init_with_level(config.log_level()).chain_err(|| "simple logger failed?")?;
    config.validate()?;
    write_config_to_file("converted.toml", config.into_toml()?)
}

fn write_config_to_file(filename: &str, config: String) -> Result<()> {
    fs::write(filename, config).chain_err(|| "couldn't write file")
}

/*
use std::sync::{Arc, Mutex, Condvar};

pub fn main_loop(config: configuration::Configuration) {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    register_signal_handlers();
    let scanner = Scanner::new(config);
    scanner.start();
    info!("Shutdown complete");
}
*/
