use crate::errors::*;
use ini::{ini, Ini};
use log;
use rollbar_rust::types::Level;
use std::collections::HashMap;
use std::mem;
use toml;

const DEFAULT: &'static str = "DEFAULT";

#[derive(Debug)]
pub struct Configuration {
    file: String,
    dry_run: bool,
    skip_to_end: bool,
    log_level: LogLevel,
    global: GlobalConfiguration,
    apps: HashMap<String, App>,
    formats: HashMap<String, Format>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum LogLevel {
    Debug,
    Info,
    Warning,
}

impl Default for LogLevel {
    fn default() -> LogLevel {
        LogLevel::Info
    }
}

impl Into<log::Level> for LogLevel {
    fn into(self) -> log::Level {
        match self {
            LogLevel::Debug => log::Level::Debug,
            LogLevel::Info => log::Level::Info,
            LogLevel::Warning => log::Level::Warn,
        }
    }
}

#[derive(Debug, Default)]
struct InnerConfiguration {
    file: String,
    dry_run: bool,
    skip_to_end: bool,
    log_level: LogLevel,
    apps: HashMap<String, App>,
    formats: HashMap<String, Format>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TomlConfiguration {
    #[serde(rename = "DEFAULT")]
    default: GlobalConfiguration,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    app: Vec<App>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    format: Vec<Format>,
}

impl TomlConfiguration {
    fn consume(mut self, inner: &mut InnerConfiguration, global: &mut GlobalConfiguration) {
        for app in self.app {
            inner.apps.insert(app.name.clone(), app);
        }
        for format in self.format {
            inner.formats.insert(format.name.clone(), format);
        }
        mem::swap(&mut self.default, global);
    }
}

impl From<Configuration> for TomlConfiguration {
    fn from(mut conf: Configuration) -> TomlConfiguration {
        TomlConfiguration {
            default: conf.global,
            app: conf.apps.drain().map(|(_, v)| v).collect(),
            format: conf.formats.drain().map(|(_, v)| v).collect(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GlobalConfiguration {
    endpoint: String,
    #[serde(rename = "timeout")]
    timeout_secs: u64,
    log_level: Level,
    statefile: String,
    sleep_time: u64,
    ext_whitelist: Vec<String>,
    ext_blacklist: Vec<String>,
    targets: Vec<String>,
    blacklist: Vec<String>,
    scrub_regex_patterns: Vec<String>,
    delete_processed_files: bool,
    filter_chr_attr_sequences: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct App {
    name: String,
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    timeout_secs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ext_whitelist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ext_blacklist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blacklist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scrub_regex_patterns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_processed_files: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_chr_attr_sequences: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_log_level: Option<Level>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_format: Option<LogFormat>,
}

impl App {
    fn new(name: &str) -> Self {
        App {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn validate(&self) -> Result<()> {
        if let Some(ts) = &self.targets {
            if ts.len() == 0 {
                bail!(ErrorKind::MissingTargets(self.name.clone()));
            }
        } else {
            bail!(ErrorKind::MissingTargets(self.name.clone()));
        }
        if let Some(ps) = &self.params {
            if !ps.contains_key("access_token") {
                bail!(ErrorKind::MissingAccessToken(self.name.clone()));
            }
        } else {
            bail!(ErrorKind::MissingAccessToken(self.name.clone()));
        }
        Ok(())
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LogFormat {
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patterns: Option<Vec<Vec<String>>>,
}

impl LogFormat {
    fn has_data(&self) -> bool {
        self.default.is_some() || self.patterns.is_some()
    }

    fn wrap_if_data(self) -> Option<Self> {
        if self.has_data() {
            Some(self)
        } else {
            None
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Format {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    datefmt: Option<String>,
}

impl Format {
    fn new(name: &str) -> Self {
        Format {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl Default for GlobalConfiguration {
    fn default() -> GlobalConfiguration {
        GlobalConfiguration {
            endpoint: "https://api.rollbar.com/api/1/item/".to_owned(),
            timeout_secs: 3,
            log_level: Level::Info,
            statefile: "/var/cache/rollbar-agent.state".to_owned(),
            sleep_time: 10,
            ext_whitelist: vec!["log".to_owned(), "rollbar".to_owned()],
            ext_blacklist: vec![],
            targets: vec![],
            blacklist: vec![],
            scrub_regex_patterns: vec![],
            delete_processed_files: false,
            filter_chr_attr_sequences: false,
        }
    }
}

pub struct ConfigurationBuilder {
    c: InnerConfiguration,
    g: GlobalConfiguration,
}

impl Configuration {
    pub fn builder() -> ConfigurationBuilder {
        ConfigurationBuilder::new()
    }

    fn build(inner: InnerConfiguration, global: GlobalConfiguration) -> Self {
        Configuration {
            file: inner.file,
            dry_run: inner.dry_run,
            skip_to_end: inner.skip_to_end,
            log_level: inner.log_level,
            global,
            apps: inner.apps,
            formats: inner.formats,
        }
    }

    pub fn to_toml(self) -> Result<String> {
        debug!("Starting conversion to TOML");
        let conf = TomlConfiguration::from(self);
        toml::to_string(&conf).chain_err(|| "bad toml data")
    }

    pub fn validate(&self) -> Result<()> {
        for (_name, app) in &self.apps {
            app.validate()?;
        }
        Ok(())
    }

    pub fn log_level(&self) -> log::Level {
        self.log_level.clone().into()
    }
}

impl ConfigurationBuilder {
    pub fn new() -> Self {
        ConfigurationBuilder {
            c: InnerConfiguration::default(),
            g: GlobalConfiguration::default(),
        }
    }

    pub fn file<T: Into<String>>(mut self, f: T) -> Self {
        self.c.file = f.into();
        self
    }

    pub fn dry_run(mut self, d: bool) -> Self {
        self.c.dry_run = d;
        self
    }

    pub fn skip_to_end(mut self, s: bool) -> Self {
        self.c.skip_to_end = s;
        self
    }

    pub fn verbose(&mut self) -> &mut Self {
        self.c.log_level = LogLevel::Debug;
        self
    }

    pub fn quiet(&mut self) -> &mut Self {
        self.c.log_level = LogLevel::Warning;
        self
    }

    pub fn process_file(mut self) -> Result<Self> {
        if self.c.file.ends_with(".conf") {
            self.process_ini_file()?;
        } else if self.c.file.ends_with(".toml") {
            self.process_toml_file()?;
        }
        Ok(self)
    }

    pub fn build(self) -> Configuration {
        Configuration::build(self.c, self.g)
    }

    fn process_ini_file(&mut self) -> Result<()> {
        let conf = Ini::load_from_file(&self.c.file).chain_err(|| ErrorKind::ProcessFileFailed)?;
        self.process_defaults(&conf)?;
        for (section, prop) in &conf {
            match section {
                Some(val) => {
                    if val == DEFAULT {
                        continue;
                    }
                    if val.starts_with("app:") {
                        self.process_app(&val[4..], prop)?;
                    } else if val.starts_with("format:") {
                        self.process_format(&val[7..], prop);
                    }
                }
                None => {}
            }
        }
        Ok(())
    }

    fn process_toml_file(&mut self) -> Result<()> {
        use std::fs::File;
        use std::io::prelude::*;
        let mut input = String::new();
        File::open(&self.c.file)
            .and_then(|mut f| f.read_to_string(&mut input))
            .chain_err(|| ErrorKind::ProcessFileFailed)?;
        let conf: TomlConfiguration =
            toml::from_str(&input).chain_err(|| ErrorKind::ProcessFileFailed)?;
        conf.consume(&mut self.c, &mut self.g);
        Ok(())
    }

    fn process_defaults(&mut self, conf: &Ini) -> Result<()> {
        if let Some(props) = conf.section(Some(DEFAULT)) {
            for (key, value) in props {
                match &key[..] {
                    "endpoint" => self.g.endpoint = value.to_string(),
                    "timeout" => {
                        self.g.timeout_secs = value
                            .parse::<u64>()
                            .chain_err(|| ErrorKind::BadInput("timeout", value.clone()))?
                    }
                    "log_level" => self.g.log_level = Level::from(&value[..]),
                    "statefile" => self.g.statefile = value.to_string(),
                    "sleep_time" => {
                        self.g.sleep_time = value
                            .parse::<u64>()
                            .chain_err(|| ErrorKind::BadInput("sleep_time", value.clone()))?
                    }
                    "ext_whitelist" => self.g.ext_whitelist = self.convert_to_list(value),
                    "ext_blacklist" => self.g.ext_blacklist = self.convert_to_list(value),
                    "targets" => self.g.targets = self.convert_to_list(value),
                    "blacklist" => self.g.blacklist = self.convert_to_list(value),
                    "scrub_regex_patterns" => {
                        self.g.scrub_regex_patterns = self.convert_to_list(value)
                    }
                    "delete_processed_files" => {
                        self.g.delete_processed_files = self.convert_to_bool(value)
                    }
                    "filter_chr_attr_sequences" => {
                        self.g.filter_chr_attr_sequences = self.convert_to_bool(value)
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn convert_to_list(&self, value: &String) -> Vec<String> {
        let parts: Vec<_> = value.split_whitespace().collect();
        parts.iter().map(|s| s.to_string()).collect()
    }

    fn convert_to_bool(&self, value: &String) -> bool {
        match &value[..] {
            "false" | "False" | "FALSE" | "no" | "No" | "NO" => false,
            _ => true,
        }
    }

    fn convert_to_pair_list(&self, value: &String) -> Result<Vec<Vec<String>>> {
        let mut iter = value.split_whitespace();
        let mut result = Vec::new();
        loop {
            match (iter.next(), iter.next()) {
                (Some(regex), Some(name)) => {
                    result.push(vec![regex.to_string(), name.to_string()]);
                }
                (None, None) => break,
                _ => return Err(ErrorKind::BadInput("log_format.patterns", value.clone()).into()),
            }
        }
        Ok(result)
    }

    fn process_format(&mut self, name: &str, props: &ini::Properties) {
        let mut format = Format::new(name);
        for (key, value) in props {
            match &key[..] {
                "type" => format.ty = value.to_string(),
                "format" => format.format = value.to_string(),
                "datefmt" => format.datefmt = Some(value.to_string()),
                _ => {}
            }
        }
        self.c.formats.insert(name.to_owned(), format);
    }

    fn process_app(&mut self, name: &str, props: &ini::Properties) -> Result<()> {
        let mut app = App::new(name);
        let mut params = HashMap::new();
        let mut log_format = LogFormat::default();
        for (key, value) in props {
            match &key[..] {
                "timeout" => {
                    app.timeout_secs = Some(
                        value
                            .parse::<u64>()
                            .chain_err(|| ErrorKind::BadInput("timeout", value.clone()))?,
                    )
                }
                "endpoint" => app.endpoint = Some(value.to_string()),
                "ext_whitelist" => app.ext_whitelist = Some(self.convert_to_list(value)),
                "ext_blacklist" => app.ext_blacklist = Some(self.convert_to_list(value)),
                "targets" => app.targets = Some(self.convert_to_list(value)),
                "blacklist" => app.blacklist = Some(self.convert_to_list(value)),
                "scrub_regex_patterns" => {
                    app.scrub_regex_patterns = Some(self.convert_to_list(value))
                }
                "delete_processed_files" => {
                    app.delete_processed_files = Some(self.convert_to_bool(value))
                }
                "filter_chr_attr_sequences" => {
                    app.filter_chr_attr_sequences = Some(self.convert_to_bool(value))
                }
                "min_log_level" => app.min_log_level = Some(Level::from(&value[..])),
                _ => {
                    if key.starts_with("params.") {
                        params.insert((&key[7..]).to_string(), value.to_string());
                    } else if key.starts_with("log_format.default") {
                        log_format.default = Some(value.to_string());
                    } else if key.starts_with("log_format.patterns") {
                        log_format.patterns = Some(self.convert_to_pair_list(value)?);
                    }
                }
            }
        }
        if !params.is_empty() {
            app.params = Some(params);
        }
        app.log_format = log_format.wrap_if_data();
        self.c.apps.insert(name.to_owned(), app);
        Ok(())
    }
}
