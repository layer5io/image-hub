mod json_parse;
mod rate_limiter;

use json_parse::{JsonPath, RateLimiterJson, Rule};
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use rate_limiter::RateLimiter;
use serde::Deserialize;

use std::time::SystemTime;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(UpstreamCallRoot {
            config_json: String::new(),
        })
    });
}

static CORS_HEADERS: [(&str, &str); 5] = [
    ("Powered-By", "proxy-wasm"),
    ("Access-Control-Allow-Origin", "*"),
    ("Access-Control-Allow-Methods", "*"),
    ("Access-Control-Allow-Headers", "*"),
    ("Access-Control-Max-Age", "3600"),
];

#[derive(Deserialize, Debug)]
struct Data {
    username: String,
    plan: String,
}

#[derive(Debug)]
struct UpstreamCall {
    config_json: Vec<JsonPath>,
}

impl UpstreamCall {
    fn new(json_str: &String) -> Self {
        let mut json: Vec<JsonPath> = serde_json::from_str(json_str).unwrap();
        json.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
        Self { config_json: json }
    }

    fn get_paths(json: &Vec<JsonPath>) -> Vec<String> {
        let mut allowed_paths: Vec<String> = json.iter().map(|e| e.name.clone()).collect();
        allowed_paths.sort();
        allowed_paths
    }

    fn is_rate_limiter(&self, path: String) -> Option<Vec<RateLimiterJson>> {
        // only meant to check if rule type is rate limiter
        let comp_type = Rule::RateLimiter(Vec::new());
        let rule_vec = self
            .config_json
            .binary_search_by(|x| x.name.cmp(&path))
            .unwrap();
        if std::mem::discriminant(&self.config_json[rule_vec].rule)
            == std::mem::discriminant(&comp_type)
        {
            if let Rule::RateLimiter(plans_vec) = &self.config_json[rule_vec].rule {
                return Some(plans_vec.to_vec());
            }
        }
        return None;
    }
}

impl Context for UpstreamCall {}

impl HttpContext for UpstreamCall {
    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {
        if let Some(method) = self.get_http_request_header(":method") {
            if method == "OPTIONS" {
                self.send_http_response(204, CORS_HEADERS.to_vec(), None);
                return Action::Pause;
            }
        }
        if let Some(path) = self.get_http_request_header(":path") {
            if UpstreamCall::get_paths(&self.config_json)
                .binary_search(&path)
                .is_ok()
            {
                return Action::Continue;
            }
        }

        if let Some(header) = self.get_http_request_header("Authorization") {
            let test = self.is_rate_limiter(self.get_http_request_header(":path").unwrap());
            proxy_wasm::hostcalls::log(LogLevel::Warn, format!("test2: {:?}", test).as_str()).ok();
            if let Some(plans_vec) =
                self.is_rate_limiter(self.get_http_request_header(":path").unwrap())
            {
                if let Ok(token) = base64::decode(header) {
                    let obj: Data = serde_json::from_slice(&token).unwrap();

                    proxy_wasm::hostcalls::log(LogLevel::Debug, format!("Obj {:?}", obj).as_str())
                        .ok();

                    let curr = self.get_current_time();
                    let tm = curr.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                    let mn = (tm.as_secs() / 60) % 60;
                    let _sc = tm.as_secs() % 60;
                    let mut rl = RateLimiter::get(&obj.username, &obj.plan);

                    let mut headers = CORS_HEADERS.to_vec();
                    let count: String;

                    let limit = plans_vec
                        .into_iter()
                        .filter(|x| x.identifier == obj.plan)
                        .map(|x| x.limit)
                        .collect::<Vec<u32>>()[0];

                    if rl.update(mn as i32) > limit {
                        count = rl.count.to_string();
                        headers
                            .append(&mut vec![("x-rate-limit", &count), ("x-app-user", &rl.key)]);
                        self.send_http_response(429, headers, Some(b"Limit exceeded.\n"));
                        rl.set();
                        return Action::Pause;
                    }
                    proxy_wasm::hostcalls::log(LogLevel::Debug, format!("Obj {:?}", &rl).as_str())
                        .ok();
                    count = rl.count.to_string();
                    rl.set();
                    headers.append(&mut vec![("x-rate-limit", &count), ("x-app-user", &rl.key)]);
                    self.send_http_response(200, headers, Some(b"All Good!\n"));
                    return Action::Continue;
                }
            }
        }
        self.send_http_response(401, CORS_HEADERS.to_vec(), Some(b"Unauthorized\n"));
        Action::Pause
    }

    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        self.set_http_response_header("x-app-serving", Some("rate-limit-filter"));
        proxy_wasm::hostcalls::log(LogLevel::Debug, format!("RESPONDING").as_str()).ok();
        Action::Continue
    }
}

struct UpstreamCallRoot {
    config_json: String,
}

impl Context for UpstreamCallRoot {}
impl RootContext for UpstreamCallRoot {
    //TODO: Revisit this once the read only feature is released in Istio 1.10
    fn on_vm_start(&mut self, _: usize) -> bool {
        if let Some(config_bytes) = self.get_configuration() {
            let config_str = String::from_utf8(config_bytes).unwrap();
            let config_b64 = base64::decode(config_str).unwrap();
            self.config_json = String::from_utf8(config_b64).unwrap();
        }
        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(UpstreamCall::new(&self.config_json)))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}
