use rollbar_rust::types::{DataBuilder, Item};
use rollbar_rust::{constants, Configuration, HttpTransport, Transport, Uuid};
use std::time::Duration;

pub struct Rollbar {
    conf: Configuration,
    transport: HttpTransport,
}

impl Rollbar {
    pub fn configuration_from_file(filename: &str) -> Result<Configuration, String> {
        use std::fs::File;
        use std::io::prelude::*;
        let mut input = String::new();
        File::open(&filename)
            .and_then(|mut f| f.read_to_string(&mut input))
            .map_err(|err| err.to_string())?;
        toml::from_str(&input).map_err(|err| err.to_string())
    }

    pub fn new(conf: Configuration) -> Self {
        let transport = HttpTransport::new(&conf);
        Rollbar { conf, transport }
    }

    pub fn send(&self, builder: DataBuilder) {
        let data = builder
            .notifier(constants::NOTIFIER.clone())
            .platform(constants::PLATFORM)
            .uuid(Uuid::new())
            .build();

        let item = Item::builder()
            .access_token(self.conf.access_token.clone().unwrap())
            .data(data)
            .build();
        self.transport.send(item);
    }

    pub fn shutdown(&self) {
        self.transport.shutdown(Duration::from_secs(5));
    }
}
