use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct JsonPath {
    pub name: String,
    rule: Rule,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
#[serde(tag = "RuleType", content = "parameters")]
pub enum Rule {
    RateLimiter(Vec<RateLimiter>),
    None,
}

#[derive(Clone, Deserialize, Debug)]
pub struct RateLimiter {
    pub identifier: String,
    pub limit: u32,
}
