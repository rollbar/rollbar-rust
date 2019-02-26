use types::Notifier;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

include!(concat!(env!("OUT_DIR"), "/constants.gen.rs"));

lazy_static! {
    pub static ref NOTIFIER: Notifier = Notifier {
        name: Some("rollbar-rust".into()),
        version: Some(VERSION.into()),
    };
}
