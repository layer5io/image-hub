mod rate_limiter;
mod yaml_parse;

use rate_limiter::RateLimiter;
use yaml_parse::JsonPath;

use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
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
    //data: Vec<JsonPath>,
//paths: Vec<String>,
}

impl UpstreamCall {
    fn new() -> Self {
        /*Parsing JSON
        let file_path = Path::new("filter.json");
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let json: Vec<JsonPath> = serde_json::from_reader(reader).unwrap();
        let path_vec = UpstreamCall::get_paths(&json);
        */
        Self {
            //data: json,
            //paths: path_vec,
        }
    }

    fn get_paths(json: &Vec<JsonPath>) -> Vec<String> {
        let mut allowed_paths: Vec<String> = json.iter().map(|e| e.name.clone()).collect();
        allowed_paths.sort();
        allowed_paths
    }
}

static ALLOWED_PATHS: [&str; 4] = ["/auth", "/signup", "/upgrade", "/pull"];
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
        proxy_wasm::hostcalls::log(
            LogLevel::Trace,
            format!("Obj {:?}", self.get_configuration()).as_str(),
        )
        .ok();
        if let Some(method) = self.get_http_request_header(":method") {
            if method == "OPTIONS" {
                self.send_http_response(204, CORS_HEADERS.to_vec(), None);
                return Action::Pause;
            }
        }

        if let Some(path) = self.get_http_request_header(":path") {
            if ALLOWED_PATHS.binary_search(&path.as_str()).is_ok() {
                return Action::Continue;
            }
        }
        /*
        if let Some(path) = self.get_http_request_header(":path") {
            if self.paths.binary_search(&path).is_ok() {
                return Action::Continue;
            }
        }*/
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
                headers.append(&mut vec![
                    ("x-rate-limit", &count),
                    ("x-app-user", &rl.key),
                    ("json", "parsed"),
                ]);
                self.send_http_response(200, headers, Some(b"All Good!\n"));
                return Action::Continue;
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

impl UpstreamCall {
    // fn retrieve_rl(&self) -> RateLimiter {
    // }
}

impl Context for UpstreamCall {}
impl RootContext for UpstreamCall {}
