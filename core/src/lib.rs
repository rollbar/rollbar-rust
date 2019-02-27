#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate builder_derive;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate uuid;

mod configuration;
pub mod constants;
mod errors;
pub mod types;
pub use log::Level;
mod transport;

pub use crate::configuration::Configuration;
pub use crate::transport::{HttpTransport, Transport};

pub struct Uuid(uuid::Uuid);

impl Uuid {
    pub fn new() -> Self {
        Uuid(uuid::Uuid::new_v4())
    }
}

impl Into<String> for Uuid {
    fn into(self) -> String {
        format!("{}", self.0.to_hyphenated())
        //format!("{}", self.0.to_simple())
    }
}

use std::fmt;
impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
