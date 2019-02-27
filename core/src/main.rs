#[macro_use] extern crate log;

use rollbar_rust::constants;
use rollbar_rust::types::*;
use rollbar_rust::Uuid;
use rollbar_rust::{Configuration, HttpTransport, Transport};
use std::time::Duration;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let conf = make_configuration();
    let transport = HttpTransport::new(&conf);
    let item = make_item(&conf);
    transport.send(item);
    transport.shutdown(Duration::from_secs(5));
}

fn make_configuration() -> Configuration {
    let mut conf = Configuration::default();
    conf.access_token = Some("a4ced289a17c42928fb4b7fdba5f2ce0".to_owned());
    conf
}

fn make_item(configuration: &Configuration) -> Item {
    let message = Message::builder().body("Hello, Rust").build();

    let body = Body::builder().message(message).build();

    let server = Server::builder()
        .cpu(constants::ARCH)
        .host("localhost")
        .build();

    let uuid = Uuid::new();
    info!("Building: {}", uuid);

    let data = Data::builder()
        .body(body)
        .environment("testing")
        .level(Level::Error)
        .platform(constants::PLATFORM)
        .language("rust")
        .server(server)
        .person(Person::builder().id("42").username("bob").build())
        .notifier(constants::NOTIFIER.clone())
        .uuid(uuid)
        .build();

    Item::builder()
        .access_token(configuration.access_token.clone().unwrap())
        .data(data)
        .build()
}
