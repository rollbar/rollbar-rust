mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::HttpTransport;

//#[cfg(not(target_arch = "wasm32"))]
//mod threads;

//#[cfg(not(target_arch = "wasm32"))]
//pub use threads::HttpTransport;

pub const QUEUE_DEPTH: usize = 50;

use std::time::Duration;

use crate::types::Item;

pub trait Transport: Send + Sync + 'static {
    fn send(&self, item: Item);

    fn shutdown(&self, timeout: Duration) -> bool {
        let _timeout = timeout;
        true
    }
}
