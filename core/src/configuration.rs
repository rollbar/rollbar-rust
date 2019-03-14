use crate::types::Level;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Configuration {
    pub endpoint: String,
    pub access_token: Option<String>,
    pub environment: Option<String>,
    pub host: Option<String>,
    pub code_version: Option<String>,
    pub log_level: Level,
    pub timeout: u64,
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            endpoint: "https://api.rollbar.com/api/1/item/".to_owned(),
            access_token: None,
            environment: None,
            host: None,
            code_version: None,
            log_level: Level::Info,
            timeout: 10,
        }
    }
}
