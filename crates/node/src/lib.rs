use {
    neon::prelude::*,
    serde_json::Value,
    std::{collections::HashMap, time::Duration},
};

use rollbar_rust::{
    types::Level, Body, Configuration, Data, HttpTransport, Item, Message, Transport,
};

#[derive(Debug)]
pub struct Rollbar {
    transport: HttpTransport,
}

impl Finalize for Rollbar {}

impl Rollbar {
    pub fn from_config(mut cx: FunctionContext) -> JsResult<JsBox<Rollbar>> {
        let input: Handle<JsValue> = cx.argument(0)?;

        let config: Configuration =
            neon_serde2::from_value(&mut cx, input).or_else(|e| cx.throw_error(e.to_string()))?;

        let transport = HttpTransport::new(config).or_else(|e| cx.throw_error(e.to_string()))?;

        Ok(cx.boxed(Rollbar { transport }))
    }

    pub fn shutdown(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Rollbar>, _>(&mut cx)?;
        instance.transport.shutdown(Duration::from_secs(5));
        Ok(cx.undefined())
    }

    pub fn log_with<'a>(
        instance: Handle<JsBox<Self>>,
        level: Level,
        start_arg_idx: i32,
        mut cx: FunctionContext<'a>,
    ) -> JsResult<'a, JsUndefined> {
        let message: Handle<JsString> = cx.argument(start_arg_idx)?;

        let extra: Option<Handle<JsValue>> = cx.argument_opt(start_arg_idx + 1);

        let extra: HashMap<String, Value> = if let Some(extra) = extra {
            neon_serde2::from_value(&mut cx, extra).or_else(|e| cx.throw_error(e.to_string()))?
        } else {
            HashMap::new()
        };

        let message = Message::builder()
            .body(message.value(&mut cx))
            .extra(extra)
            .build();
        let body = Body::builder().message(message).build();
        let mut data = Data::default();
        data.level = Some(level);
        data.body = body;

        let config = instance.transport.config();

        let access_token = config.access_token.clone().expect("missing access token");

        let item = Item::builder()
            .access_token(access_token)
            .data(data)
            .build();

        instance.transport.send(item);

        Ok(cx.undefined())
    }

    pub fn log(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Rollbar>, _>(&mut cx)?;

        let level: Handle<JsString> = cx.argument(0)?;
        let level = Level::from(level.value(&mut cx));

        Self::log_with(instance, level, 1, cx)
    }

    pub fn debug(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Rollbar>, _>(&mut cx)?;

        Self::log_with(instance, Level::Debug, 0, cx)
    }

    pub fn info(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Rollbar>, _>(&mut cx)?;

        Self::log_with(instance, Level::Info, 0, cx)
    }

    pub fn warning(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Rollbar>, _>(&mut cx)?;

        Self::log_with(instance, Level::Warning, 0, cx)
    }

    pub fn error(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Rollbar>, _>(&mut cx)?;

        Self::log_with(instance, Level::Error, 0, cx)
    }

    pub fn critical(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let instance = cx.this().downcast_or_throw::<JsBox<Rollbar>, _>(&mut cx)?;

        Self::log_with(instance, Level::Critical, 0, cx)
    }
}

#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("fromConfig", Rollbar::from_config)?;
    cx.export_function("log", Rollbar::log)?;
    cx.export_function("debug", Rollbar::debug)?;
    cx.export_function("info", Rollbar::info)?;
    cx.export_function("warning", Rollbar::warning)?;
    cx.export_function("error", Rollbar::error)?;
    cx.export_function("critical", Rollbar::critical)?;
    cx.export_function("shutdown", Rollbar::shutdown)?;

    Ok(())
}
