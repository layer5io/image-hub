use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JsonPath {
    pub name: String,
    rule: Rule,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
#[serde(tag = "RuleType", content = "parameters")]
pub enum Rule {
    RateLimiter(Vec<RateLimiter>),
    None,
}

#[derive(Deserialize, Debug)]
pub struct RateLimiter {
    pub identifier: String,
    pub limit: u32,
}
