use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
// TODO: isaac
use std::thread;

#[derive(Serialize, Deserialize, Debug, Default, Builder)]
pub struct Item {
    pub access_token: String,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder)]
pub struct Data {
    pub body: Body,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<Client>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<Request>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<Person>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifier: Option<Notifier>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Body {
    #[serde(skip)]
    Unset,
    TraceBody {
        #[serde(skip_serializing_if = "Option::is_none")]
        telemetry: Option<Vec<Telemetry>>,
        trace: Trace,
    },
    MessageBody {
        #[serde(skip_serializing_if = "Option::is_none")]
        telemetry: Option<Vec<Telemetry>>,
        message: Message,
    },
    TraceChainBody {
        #[serde(skip_serializing_if = "Option::is_none")]
        telemetry: Option<Vec<Telemetry>>,
        trace_chain: Vec<Trace>,
    },
    CrashReportBody {
        #[serde(skip_serializing_if = "Option::is_none")]
        telemetry: Option<Vec<Telemetry>>,
        crash_report: CrashReport,
    },
}

impl Default for Body {
    fn default() -> Body {
        Body::Unset
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Builder)]
pub struct Message {
    pub body: String,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder)]
pub struct Telemetry {
    pub level: Level,
    #[serde(rename = "type")]
    pub telemetry_type: String,
    pub source: String,
    pub timestamp_ms: u64,
    pub body: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder)]
pub struct Trace {
    pub frames: Vec<Frame>,
    pub exception: Exception,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder)]
pub struct Frame {
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineno: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colno: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code")]
    pub function_code_line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argspec: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub varargspec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywordspec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locals: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Builder)]
pub struct Exception {
    pub class: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Default for Exception {
    fn default() -> Self {
        Exception {
            class: thread::current().name().unwrap_or("unnamed").to_owned(),
            message: None,
            description: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Builder)]
pub struct CrashReport {
    pub raw: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder)]
pub struct Person {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder)]
pub struct Notifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "GET")]
    pub get: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "POST")]
    pub post: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ip: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder)]
pub struct Server {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_version: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder)]
pub struct Client {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub javascript: Option<Javascript>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, Builder)]
pub struct Javascript {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_map_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guess_uncaught_frames: Option<bool>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl Level {
    pub fn python_level(self) -> u64 {
        match self {
            Level::Debug => 10,
            Level::Info => 20,
            Level::Warning => 30,
            Level::Error => 40,
            Level::Critical => 50,
        }
    }
}

impl<'a> From<&'a str> for Level {
    fn from(s: &'a str) -> Level {
        match s {
            "critical" | "crit" | "criti" | "CRITICAL" | "CRIT" | "CRITI" => Level::Critical,
            "warning" | "warn" | "warni" | "WARNING" | "WARN" | "WARNI" => Level::Warning,
            "info" | "INFO" => Level::Info,
            "debug" | "DEBUG" => Level::Debug,
            _ => Level::Error,
        }
    }
}

impl ToString for Level {
    fn to_string(&self) -> String {
        match *self {
            Level::Critical => "critical",
            Level::Error => "error",
            Level::Warning => "warning",
            Level::Info => "info",
            Level::Debug => "debug",
        }
        .to_string()
    }
}

impl Default for Level {
    fn default() -> Level {
        Level::Error
    }
}

impl<'de> ::serde::Deserialize<'de> for Level {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Level;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("one of the level variants")
            }

            fn visit_str<E>(self, value: &str) -> Result<Level, E>
            where
                E: ::serde::de::Error,
            {
                Ok(Level::from(value))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Default)]
pub struct BodyBuilder {
    telemetry: Option<Vec<Telemetry>>,
}

pub struct TraceBodyBuilder {
    telemetry: Option<Vec<Telemetry>>,
    node: Trace,
}

pub struct MessageBodyBuilder {
    telemetry: Option<Vec<Telemetry>>,
    node: Message,
}

pub struct CrashReportBodyBuilder {
    telemetry: Option<Vec<Telemetry>>,
    node: CrashReport,
}

pub struct TraceChainBodyBuilder {
    telemetry: Option<Vec<Telemetry>>,
    trace_chain: Vec<Trace>,
}

impl Body {
    pub fn builder() -> BodyBuilder {
        BodyBuilder::new()
    }
}

impl BodyBuilder {
    pub fn new() -> Self {
        BodyBuilder { telemetry: None }
    }

    pub fn telemetry(mut self, val: Vec<Telemetry>) -> Self {
        self.telemetry = Some(val);
        self
    }

    pub fn push_telemetry(mut self, item: Telemetry) -> Self {
        self.telemetry.get_or_insert_with(|| vec![]).push(item);
        self
    }

    pub fn message(self, message: Message) -> MessageBodyBuilder {
        MessageBodyBuilder::new(self.telemetry, message)
    }

    pub fn trace(self, trace: Trace) -> TraceBodyBuilder {
        TraceBodyBuilder::new(self.telemetry, trace)
    }

    pub fn trace_chain(self, trace_chain: Vec<Trace>) -> TraceChainBodyBuilder {
        TraceChainBodyBuilder::new(self.telemetry, trace_chain)
    }

    pub fn push_trace(self, trace: Trace) -> TraceChainBodyBuilder {
        TraceChainBodyBuilder::new(self.telemetry, vec![trace])
    }
}

macro_rules! body_builder {
    ($inner:ident, $name:ident, $body:path, $b:ident) => {
        impl $b {
            pub fn new(telemetry: Option<Vec<Telemetry>>, node: $inner) -> Self {
                $b { telemetry, node }
            }

            pub fn telemetry(mut self, val: Vec<Telemetry>) -> Self {
                self.telemetry = Some(val);
                self
            }

            pub fn push_telemetry(mut self, item: Telemetry) -> Self {
                self.telemetry.get_or_insert_with(|| vec![]).push(item);
                self
            }

            pub fn $name(mut self, node: $inner) -> Self {
                self.node = node;
                self
            }

            pub fn build(self) -> Body {
                $body {
                    telemetry: self.telemetry,
                    $name: self.node,
                }
            }
        }
    };
}

body_builder!(Trace, trace, Body::TraceBody, TraceBodyBuilder);
body_builder!(Message, message, Body::MessageBody, MessageBodyBuilder);
body_builder!(
    CrashReport,
    crash_report,
    Body::CrashReportBody,
    CrashReportBodyBuilder
);

impl TraceChainBodyBuilder {
    pub fn new(telemetry: Option<Vec<Telemetry>>, trace_chain: Vec<Trace>) -> Self {
        TraceChainBodyBuilder {
            telemetry,
            trace_chain,
        }
    }

    pub fn telemetry(mut self, val: Vec<Telemetry>) -> Self {
        self.telemetry = Some(val);
        self
    }

    pub fn push_telemetry(mut self, item: Telemetry) -> Self {
        self.telemetry.get_or_insert_with(|| vec![]).push(item);
        self
    }

    pub fn trace_chain(mut self, trace_chain: Vec<Trace>) -> Self {
        self.trace_chain = trace_chain;
        self
    }

    pub fn push_trace(mut self, trace: Trace) -> Self {
        self.trace_chain.push(trace);
        self
    }

    pub fn build(self) -> Body {
        Body::TraceChainBody {
            telemetry: self.telemetry,
            trace_chain: self.trace_chain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_from_str() {
        assert_eq!(Level::from("oops"), Level::Error);
        assert_eq!(Level::from("warn"), Level::Warning);
        assert_eq!(Level::from("warning"), Level::Warning);
        assert_eq!(Level::from("debug"), Level::Debug);
    }

    #[test]
    fn test_item_builder() {
        let message = Message::builder().body("Hello, World!").build();
        let body = Body::builder().message(message).build();
        let mut data = Data::default();
        data.body = body;
        let item = Item::builder().access_token("abc123").data(data).build();

        match item.data.body {
            Body::MessageBody { message, .. } => {
                assert_eq!(message.body, "Hello, World!");
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_trace_chain_builder() {
        let frame1 = Frame::builder().lineno(99).build();
        let frame2 = Frame::builder().lineno(2).build();
        let mut trace1 = Trace::default();
        trace1.frames.push(frame1);
        let mut trace2 = Trace::default();
        trace2.frames.push(frame2);
        let trace_chain = Body::builder()
            .push_trace(trace1)
            .push_trace(trace2)
            .build();

        match trace_chain {
            Body::TraceChainBody { trace_chain, .. } => {
                assert_eq!(trace_chain[0].frames[0].lineno, Some(99));
                assert_eq!(trace_chain[1].frames[0].lineno, Some(2));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_frame_builder() {
        let frame = Frame::builder()
            .filename(file!())
            .lineno(42)
            .method("hello")
            .function_code_line("hello(99)")
            .build();

        assert_eq!(frame.lineno.unwrap(), 42);
        assert_eq!(frame.method.unwrap(), "hello");
        assert_eq!(frame.colno, None);
        assert_eq!(frame.filename, "core/src/types.rs");
    }

    #[test]
    fn test_message_builder() {
        let message = Message::builder()
            .body(format!("Whoa there {}", 42))
            .build();

        assert_eq!(message.body, "Whoa there 42");
    }

    #[test]
    fn test_javascript_builder() {
        let mut extra = HashMap::new();
        extra.insert("stuff".into(), Value::Bool(true));
        let javascript = Javascript::builder()
            .extra(extra)
            .guess_uncaught_frames(true)
            .build();

        assert_eq!(javascript.extra.get("stuff"), Some(&Value::Bool(true)));
        assert!(javascript.guess_uncaught_frames.unwrap());
        assert_eq!(javascript.source_map_enabled, None);
    }
}
