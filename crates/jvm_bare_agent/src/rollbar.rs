use rollbar_rust::types::{DataBuilder, Item, Notifier, Server};
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
            .notifier(
                Notifier::builder()
                    .name("rollbar-jvm-agent")
                    .version(constants::VERSION)
                    .build(),
            )
            .platform(constants::PLATFORM)
            .uuid(Uuid::new())
            .maybe_environment(self.conf.environment.clone())
            .server(
                Server::builder()
                    .cpu(constants::ARCH)
                    .maybe_host(self.conf.host.clone())
                    .maybe_code_version(self.conf.code_version.clone())
                    .build(),
            )
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
