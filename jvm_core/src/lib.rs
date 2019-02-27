#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;
extern crate rollbar_rust;

pub mod env;
pub mod errors;
pub mod exceptions;
pub mod jni;

#[cfg_attr(feature = "cargo-clippy", allow(clippy::all))]
pub mod jvmti;

pub use crate::env::ExceptionCallbackFn;

pub use error_chain::bail;
