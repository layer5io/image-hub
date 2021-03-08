use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
pub struct JsonPath {
    pub name: String,
    pub rule: Rule,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all(deserialize = "kebab-case"))]
#[serde(tag = "ruleType", content = "parameters")]
pub enum Rule {
    RateLimiter(Vec<RateLimiterJson>),
    None,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
pub struct RateLimiterJson {
    pub identifier: String,
    pub limit: u32,
}
