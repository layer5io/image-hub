mod rate_limiter;
mod yaml_parse;

use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use rate_limiter::RateLimiter;
use serde::Deserialize;
use yaml_parse::JsonPath;

use std::time::SystemTime;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_http_context(|_context_id, _root_context_id| -> Box<dyn HttpContext> {
        Box::new(UpstreamCall::new())
    });
}

#[derive(Debug)]
struct UpstreamCall {
    data: Vec<JsonPath>,
    paths: Vec<String>,
    test: String,
}

impl UpstreamCall {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            paths: Vec::new(),
            test: String::from("default"),
        }
    }

    fn get_paths(json: &Vec<JsonPath>) -> Vec<String> {
        let mut allowed_paths: Vec<String> = json.iter().map(|e| e.name.clone()).collect();
        allowed_paths.sort();
        allowed_paths
    }
}

//static ALLOWED_PATHS: [&str; 4] = ["/auth", "/signup", "/upgrade", "/pull"];
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

impl HttpContext for UpstreamCall {
    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {
        if let Some(method) = self.get_http_request_header(":method") {
            if method == "OPTIONS" {
                self.send_http_response(204, CORS_HEADERS.to_vec(), None);
                return Action::Pause;
            }
        }
        /*
        if let Some(path) = self.get_http_request_header(":path") {
            if ALLOWED_PATHS.binary_search(&path.as_str()).is_ok() {
                return Action::Continue;
            }
        }
        */
        if let Some(path) = self.get_http_request_header(":path") {
            if self.paths.binary_search(&path).is_ok() {
                return Action::Continue;
            }
        }
        if let Some(header) = self.get_http_request_header("Authorization") {
            if let Ok(token) = base64::decode(header) {
                let obj: Data = serde_json::from_slice(&token).unwrap();
                proxy_wasm::hostcalls::log(LogLevel::Debug, format!("Obj {:?}", obj).as_str()).ok();
                let curr = self.get_current_time();
                let tm = curr.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                let mn = (tm.as_secs() / 60) % 60;
                let _sc = tm.as_secs() % 60;
                let mut rl = RateLimiter::get(obj.username, obj.plan);

                let mut headers = CORS_HEADERS.to_vec();
                let count: String;

                if !rl.update(mn as i32) {
                    count = rl.count.to_string();
                    headers.append(&mut vec![("x-rate-limit", &count), ("x-app-user", &rl.key)]);
                    self.send_http_response(429, headers, Some(b"Limit exceeded.\n"));
                    rl.set();
                    return Action::Pause;
                }
                proxy_wasm::hostcalls::log(LogLevel::Debug, format!("Obj {:?}", &rl).as_str()).ok();
                count = rl.count.to_string();
                rl.set();
                headers.append(&mut vec![("x-rate-limit", &count), ("x-app-user", &rl.key)]);
                self.send_http_response(200, headers, Some(b"All Good!\n"));
                return Action::Continue;
            }
        }
        self.send_http_response(401, CORS_HEADERS.to_vec(), Some(b"Unauthorized\n"));
        Action::Pause
    }

    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        let json_test = format!("{:?}", self.paths);
        let bytes_test = format!("{:?}", self.test);
        self.set_http_response_header("x-app-serving", Some("rate-limit-filter"));
        self.set_http_response_header("json_test", Some(json_test.as_str()));
        self.set_http_response_header("test", Some(bytes_test.as_str()));
        proxy_wasm::hostcalls::log(LogLevel::Debug, format!("RESPONDING").as_str()).ok();
        Action::Continue
    }
}

impl Context for UpstreamCall {}
impl RootContext for UpstreamCall {
    //TODO: Revisit this once the read only feature is released in Istio 1.10
    fn on_vm_start(&mut self, _: usize) -> bool {
        if let Some(config_bytes) = self.get_configuration() {
            self.test = format!("{:?}", config_bytes.clone());
            let config_b64: Bytes =
                base64::decode(String::from_utf8(config_bytes).unwrap()).unwrap();
            self.data = serde_json::from_str(&String::from_utf8(config_b64).unwrap()).unwrap();
            self.paths = UpstreamCall::get_paths(&self.data);
        }
        true
    }
}
