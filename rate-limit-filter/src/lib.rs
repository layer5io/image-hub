mod rate_limiter;

use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;
use rate_limiter::RateLimiter;
use std::time::SystemTime;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_http_context(|context_id, root_context_id| -> Box<dyn HttpContext> {
        Box::new(UpstreamCall::new())
    });
}

#[derive(Debug)]
struct UpstreamCall {

}

impl UpstreamCall {
    fn new() -> Self {
        return Self {

        }
    }
}

static ALLOWED_PATHS: [&str; 1] = ["/auth"];
static CORS_HEADERS: [(&str,&str);5] = [
    ("Powered-By", "proxy-wasm"),
    ("Access-Control-Allow-Origin", "*"),
    ("Access-Control-Allow-Methods", "*"),
    ("Access-Control-Allow-Headers", "*"),
    ("Access-Control-Max-Age", "3600")   
];

impl HttpContext for UpstreamCall {
    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {
        if let Some(method) = self.get_http_request_header(":method") {
            if method == "OPTIONS" {
                self.send_http_response(
                    204,
                    CORS_HEADERS.to_vec(),
                    None,
                );
                return Action::Pause
            }
        }
        if let Some(path) = self.get_http_request_header(":path") {
            if ALLOWED_PATHS.binary_search(&path.as_str()).is_ok() {
                return Action::Continue
            }
        }
        if let Some(header) = self.get_http_request_header("Authorization") {
            let curr = self.get_current_time();
            let tm = curr.duration_since(SystemTime::UNIX_EPOCH).unwrap();
            let mn = (tm.as_secs()/60)%60;
            let sc = tm.as_secs()%60;
            let mut rl = RateLimiter::get(header);
                        if !rl.update(mn as i32) {
                            self.send_http_response(
                                429,
                                CORS_HEADERS.to_vec(),
                                Some(b"Limit exceeded.\n"),
                            );
                        rl.set();
                        return Action::Pause
                        }
                        proxy_wasm::hostcalls::log(LogLevel::Debug, format!("Obj {:?}", &rl).as_str());
                        
                        rl.set();
    
            return Action::Continue
        }
        self.send_http_response(
            401,
            CORS_HEADERS.to_vec(),
            Some(b"Unauthorized\n"),
        );
        Action::Pause
    }
    
    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        proxy_wasm::hostcalls::log(LogLevel::Debug, format!("RESPONDING").as_str());
        Action::Continue
    }
}

impl UpstreamCall {
    // fn retrieve_rl(&self) -> RateLimiter {
    // }
}

impl Context for UpstreamCall {}
impl RootContext for UpstreamCall {}