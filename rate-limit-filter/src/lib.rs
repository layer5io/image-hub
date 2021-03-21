mod json_parse;
mod rate_limiter;

use json_parse::{JsonPath, RateLimiterJson, Rule};
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use rate_limiter::RateLimiter;
use serde::Deserialize;

use std::collections::HashMap;
use std::time::SystemTime;

// We need to make sure a HTTP root context is created and initialized when the filter is initialized.
// The _start() function initialises this root context
#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(UpstreamCallRoot {
            config_json: HashMap::new(),
        })
    });
}

// Defining standard CORS headers
static CORS_HEADERS: [(&str, &str); 5] = [
    ("Powered-By", "proxy-wasm"),
    ("Access-Control-Allow-Origin", "*"),
    ("Access-Control-Allow-Methods", "*"),
    ("Access-Control-Allow-Headers", "*"),
    ("Access-Control-Max-Age", "3600"),
];

// This struct is what the JWT token sent by the user will deserialize to
#[derive(Deserialize, Debug)]
struct Data {
    username: String,
    plan: String,
}

// This is the instance of a call made. It sorta derives from the root context
#[derive(Debug)]
struct UpstreamCall {
    config_json: HashMap<String, Rule>,
}

impl UpstreamCall {
    // Takes in the HashMap created in the root context mapping path name to rule type
    fn new(json_hm: &HashMap<String, Rule>) -> Self {
        Self {
            //TODO this clone is super heavy, find a way to get rid of it
            config_json: json_hm.clone(),
        }
    }

    // Check if the path specified in the incoming request's path header has rule type None.
    // Returns Option containing path name that was sent
    fn rule_is_none(&self, path: String) -> Option<String> {
        let rule = self.config_json.get(&path).unwrap();
        // checking based only on type
        if std::mem::discriminant(rule) == std::mem::discriminant(&Rule::None) {
            return Some(path);
        }
        return None;
    }

    // Check if the path specified in the incoming request's path header has rule type RateLimiter.
    // Returns Option containing vector of RateLimiterJson objects (list of plan names with limits)
    fn rule_is_rate_limiter(&self, path: String) -> Option<Vec<RateLimiterJson>> {
        let rule = self.config_json.get(&path).unwrap();
        // checking based only on type
        if std::mem::discriminant(rule) == std::mem::discriminant(&Rule::RateLimiter(Vec::new())) {
            if let Rule::RateLimiter(plans_vec) = rule {
                return Some(plans_vec.to_vec());
            }
        }
        return None;
    }
}

impl Context for UpstreamCall {}

impl HttpContext for UpstreamCall {
    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {
        // Options
        if let Some(method) = self.get_http_request_header(":method") {
            if method == "OPTIONS" {
                self.send_http_response(204, CORS_HEADERS.to_vec(), None);
                return Action::Pause;
            }
        }

        // Action for rule type: None
        if let Some(_) = self.rule_is_none(self.get_http_request_header(":path").unwrap()) {
            return Action::Continue;
        }

        // Action for rule type: RateLimiter
        if let Some(plans_vec) =
            self.rule_is_rate_limiter(self.get_http_request_header(":path").unwrap())
        {
            if let Some(header) = self.get_http_request_header("Authorization") {
                // Decoding JWT token
                if let Ok(token) = base64::decode(header) {
                    //Deserializing token
                    let obj: Data = serde_json::from_slice(&token).unwrap();

                    proxy_wasm::hostcalls::log(LogLevel::Debug, format!("Obj {:?}", obj).as_str())
                        .ok();

                    // Since the rate limit works on a rate per minute based quota, we find current time
                    let curr = self.get_current_time();
                    let tm = curr.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                    let mn = (tm.as_secs() / 60) % 60;
                    let _sc = tm.as_secs() % 60;

                    // Initialise RateLimiter object
                    let mut rl = RateLimiter::get(&obj.username, &obj.plan);

                    // Initialising headers to send back
                    let mut headers = CORS_HEADERS.to_vec();
                    let count: String;

                    // Extracting limits based on plan stated in JWT token from the corresponding RateLimiterJson
                    let limit = plans_vec
                        .into_iter()
                        .filter(|x| x.identifier == obj.plan)
                        .map(|x| x.limit)
                        .collect::<Vec<u32>>();

                    // Checking if the appropriate plan exists
                    if limit.len() != 1 {
                        self.send_http_response(
                            429,
                            headers,
                            Some(b"Invalid plan name or duplicate plan names defined.\n"),
                        );
                        return Action::Pause;
                    }

                    //Update request count in RateLimiter object, and check if it exceeds limits
                    if rl.update(mn as i32) > limit[0] {
                        count = rl.count.to_string();
                        headers
                            .append(&mut vec![("x-rate-limit", &count), ("x-app-user", &rl.key)]);
                        self.send_http_response(429, headers, Some(b"Limit exceeded.\n"));
                        rl.set();
                        return Action::Pause;
                    }
                    proxy_wasm::hostcalls::log(LogLevel::Debug, format!("Obj {:?}", &rl).as_str())
                        .ok();
                    // set the new count in headers, and proxy_wasm storage
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
    config_json: HashMap<String, Rule>,
}

impl Context for UpstreamCallRoot {}
impl<'a> RootContext for UpstreamCallRoot {
    //TODO: Revisit this once the read only feature is released in Istio 1.10
    // Get Base64 encoded JSON from envoy config file when WASM VM starts
    fn on_vm_start(&mut self, _: usize) -> bool {
        if let Some(config_bytes) = self.get_configuration() {
            // bytestring passed by VM -> String of base64 encoded JSON
            let config_str = String::from_utf8(config_bytes).unwrap();
            // String of base64 encoded JSON -> bytestring of decoded JSON
            let config_b64 = base64::decode(config_str).unwrap();
            // bytestring of decoded JSON -> String of decoded JSON
            let json_str = String::from_utf8(config_b64).unwrap();
            // Deserializing JSON String into vector of JsonPath objects
            let json_vec: Vec<JsonPath> = serde_json::from_str(&json_str).unwrap();
            // Creating HashMap of pattern ("path name", "rule type") and saving into UpstreamCallRoot object
            for i in json_vec {
                self.config_json.insert(i.name, i.rule);
            }
        }
        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        // creating UpstreamCall object for each new call
        Some(Box::new(UpstreamCall::new(&self.config_json)))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}
