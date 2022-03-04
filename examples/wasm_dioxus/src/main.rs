use dioxus::prelude::*;

use rollbar::{constants, types::Level, types::*, Configuration, HttpTransport, Transport, Uuid};

fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();

    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let (conf, _) = use_state(&cx, || make_configuration());
    let t_ref = use_ref(&cx, || {
        HttpTransport::new(conf.clone()).expect("make transport")
    });

    cx.render(rsx! (
        div { "Hello, world!" }
        button {
            onclick: move |_| {
                let item = make_item(&conf);

                t_ref.write().send(item);
            },
            "Send message!"
        }
    ))
}

const TOKEN: &str = env!("ROLLBAR_POST_ITEM_TOKEN");

fn make_configuration() -> Configuration {
    let mut conf = Configuration::default();
    conf.access_token = Some(TOKEN.to_owned());
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
