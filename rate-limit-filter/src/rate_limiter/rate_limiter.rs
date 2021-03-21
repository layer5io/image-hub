use proxy_wasm;

use proxy_wasm::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimiter {
    // Tracks time
    pub min: i32,
    // Tracks number of calls made
    pub count: u32,
    // stores a key(username according to example)
    pub key: String,
}

impl RateLimiter {
    fn new(key: &String, _plan: &String) -> Self {
        Self {
            min: -1,
            count: 0,
            key: key.clone(),
        }
    }
    // Get key and plan from proxy_wasm shared data store (username+plan name)
    pub fn get(key: &String, plan: &String) -> Self {
        if let Ok(data) = proxy_wasm::hostcalls::get_shared_data(&key.clone()) {
            if let Some(data) = data.0 {
                let data: Option<Self> = bincode::deserialize(&data).unwrap_or(None);
                if let Some(obj) = data {
                    return obj;
                }
            }
        }
        return Self::new(&key, &plan);
    }
    // Set key and plan in proxy_wasm shared data store (username+plan name)
    pub fn set(&self) {
        let target: Option<Self> = Some(self.clone());
        let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
        proxy_wasm::hostcalls::set_shared_data(&self.key.clone(), Some(&encoded), None).ok();
    }
    // Update time (minute by minute) and increment count
    pub fn update(&mut self, time: i32) -> u32 {
        if self.min != time {
            self.min = time;
            self.count = 0;
        }
        self.count += 1;
        proxy_wasm::hostcalls::log(LogLevel::Debug, format!("Obj {:?} ", self.count).as_str()).ok();
        self.count
    }
}
