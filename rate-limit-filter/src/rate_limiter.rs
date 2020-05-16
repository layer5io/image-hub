use serde::{Serialize,Deserialize};
use proxy_wasm; 

const LT: u32 = 10;

#[derive(Debug,Serialize,Deserialize,Copy,Clone)]
pub struct RateLimiter {
    RPM: u32,
    Min: i32,
    pub Count: u32, 
}

impl RateLimiter {
    pub const fn new() -> Self {
        Self {
            RPM: LT,
            Min: -1,
            Count: 0,
        }
    }
    pub fn get() -> Self {
        if let Ok(data) = proxy_wasm::hostcalls::get_shared_data("key") {
            if let Some(data) = data.0 {
                let data: Option<Self> = bincode::deserialize(&data).unwrap_or(None);
                if let Some(obj) = data {
                    return obj
                }
            }
        }
        return Self::new()
    }
    pub fn set(&self) {
        let target: Option<Self>  = Some(self.clone());
        let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
        proxy_wasm::hostcalls::set_shared_data("key", Some(&encoded), None);
    }
    pub fn update(&mut self, time: i32) -> bool {
        if self.Min != time {
            self.Min = time;
            self.Count = 0;
        }
        self.Count += 1;
        if self.Count > self.RPM {
            return false
        }
        true
    }
}