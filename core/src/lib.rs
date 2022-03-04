#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate builder_derive;
#[macro_use]
extern crate error_chain;

mod configuration;
mod errors;
mod transport;

pub mod constants;
pub mod types;
pub use log::Level;

pub use crate::configuration::Configuration;
pub use crate::transport::{HttpTransport, Transport};
pub use crate::types::*;

#[derive(Default)]
pub struct Uuid(uuid::Uuid);

impl Uuid {
    pub fn new() -> Self {
        Uuid(uuid::Uuid::new_v4())
    }
}

impl Into<String> for Uuid {
    fn into(self) -> String {
        format!("{}", self.0.to_hyphenated())
    }
}

use std::fmt;

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
