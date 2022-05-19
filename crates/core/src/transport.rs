#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use native::HttpTransport;

#[cfg(target_arch = "wasm32")]
pub use wasm::HttpTransport;

use crate::configuration::Configuration;

pub const QUEUE_DEPTH: usize = 50;

use std::time::Duration;

use crate::types::Item;

pub trait Transport: Send + Sync + 'static {
    fn send(&self, item: Item);

    fn config(&self) -> &Configuration;

    fn shutdown(&self, timeout: Duration) -> bool;
}
