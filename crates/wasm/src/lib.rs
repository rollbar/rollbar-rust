use {serde_json::Value, std::collections::HashMap, wasm_bindgen::prelude::*};

use rollbar_rust::{
    types::Level, Body, Configuration, Data, HttpTransport, Item, Message, Transport,
};

#[derive(Debug)]
#[wasm_bindgen]
pub struct Rollbar {
    transport: HttpTransport,
}

#[wasm_bindgen]
impl Rollbar {
    #[wasm_bindgen(js_name = "fromConfig")]
    pub fn from_config(input: JsValue) -> Result<Rollbar, JsValue> {
        let config: Configuration = input
            .into_serde()
            .map_err(|err| JsValue::from(format!("invalid configuration object: {}", err)))?;

        let transport = HttpTransport::new(config)
            .map_err(|err| JsValue::from(format!("could not create transport: {}", err)))?;

        Ok(Rollbar { transport })
    }

    pub fn log(&self, level: &str, message: &str, extra: JsValue) {
        let extra: Option<HashMap<String, Value>> = extra.into_serde().expect("to work");

        let message = Message::builder()
            .body(message)
            .extra(extra.unwrap_or_else(|| HashMap::new()))
            .build();
        let body = Body::builder().message(message).build();
        let mut data = Data::default();
        data.level = Some(Level::from(level));
        data.body = body;

        let config = self.transport.config();

        let access_token = config.access_token.clone().expect("missing access token");

        let item = Item::builder()
            .access_token(access_token)
            .data(data)
            .build();

        self.transport.send(item);
    }

    pub fn debug(&self, message: &str, extra: JsValue) {
        self.log("debug", message, extra)
    }

    pub fn info(&self, message: &str, extra: JsValue) {
        self.log("info", message, extra)
    }

    pub fn warning(&self, message: &str, extra: JsValue) {
        self.log("warning", message, extra)
    }

    pub fn error(&self, message: &str, extra: JsValue) {
        self.log("error", message, extra)
    }

    pub fn critical(&self, message: &str, extra: JsValue) {
        self.log("critical", message, extra)
    }
}
