#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub url_rewrite: Vec<UrLRewrite>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UrLRewrite {
    pub org: String,
    pub new: String,
}

impl Config {
    pub const GLOBAL: &str = "config.global.yaml";
}
