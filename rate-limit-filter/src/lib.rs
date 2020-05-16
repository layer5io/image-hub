mod rate_limiter;

use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;
use rate_limiter::RateLimiter;
use std::time::SystemTime;

const RPM: u32 = 10;
static mut rl: RateLimiter = RateLimiter::new(RPM);

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

impl HttpContext for UpstreamCall {
    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {
        let curr = self.get_current_time();
        let tm = curr.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let mn = (tm.as_secs()/60)%60;
        let sc = tm.as_secs()%60;
        unsafe {
            if !rl.update(sc as i32) {
                self.send_http_response(
                    429,
                    vec![("Powered-By", "proxy-wasm")],
                    Some(b"Limit exceeded.\n"),
                );
                return Action::Pause
            }
        }
        Action::Continue
    }
    
    
    
    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        Action::Continue
    }
}

impl UpstreamCall {
    // fn retrieve_rl(&self) -> RateLimiter {
        
    // }
}

impl Context for UpstreamCall {}
impl RootContext for UpstreamCall {}