use {std::collections::HashMap, wasm_bindgen::prelude::*};

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

    pub fn log(&self, level: &str, message: &str, extra: JsValue) -> Result<(), JsValue> {
        let extra: HashMap<_, _> = extra.into_serde().unwrap_or_default();
        let message = Message::builder().body(message).extra(extra).build();

        let body = Body::builder().message(message).build();

        let data = Data {
            level: Some(Level::from(level)),
            body,
            ..Data::default()
        };

        let config = self.transport.config();

        let access_token = config
            .access_token
            .as_ref()
            .ok_or(JsValue::from_str("missing access token"))?;

        let item = Item::builder()
            .access_token(access_token)
            .data(data)
            .build();

        self.transport.send(item);

        Ok(())
    }

    pub fn debug(&self, message: &str, extra: JsValue) -> Result<(), JsValue> {
        self.log("debug", message, extra)
    }

    pub fn info(&self, message: &str, extra: JsValue) -> Result<(), JsValue> {
        self.log("info", message, extra)
    }

    pub fn warning(&self, message: &str, extra: JsValue) -> Result<(), JsValue> {
        self.log("warning", message, extra)
    }

    pub fn error(&self, message: &str, extra: JsValue) -> Result<(), JsValue> {
        self.log("error", message, extra)
    }

    pub fn critical(&self, message: &str, extra: JsValue) -> Result<(), JsValue> {
        self.log("critical", message, extra)
    }
}
