use proxy_wasm;

use proxy_wasm::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimiter {
    rpm: Option<u32>,
    min: i32,
    count: u32,
    key: String,
}

impl RateLimiter {
    fn new(key: &String, plan: &String) -> Self {
        let limit = match plan.as_str() {
            "group" => Some(100),
            "user" => Some(10),
            _ => None,
        };
        Self {
            rpm: limit,
            min: -1,
            count: 0,
            key: key.clone(),
        }
    }
    pub fn get(key: String, plan: String) -> Self {
        if let Ok(data) = proxy_wasm::hostcalls::get_shared_data(&key.clone()) {
            if let Some(data) = data.0 {
                let data: Option<Self> = bincode::deserialize(&data).unwrap_or(None);
                if let Some(mut obj) = data {
                    let limit = match plan.as_str() {
                        "group" => Some(100),
                        "user" => Some(10),
                        _ => None,
                    };
                    obj.rpm = limit;
                    return obj;
                }
            }
        }
        return Self::new(&key, &plan);
    }
    pub fn set(&self) {
        let target: Option<Self> = Some(self.clone());
        let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
        proxy_wasm::hostcalls::set_shared_data(&self.key.clone(), Some(&encoded), None).ok();
    }
    pub fn update(&mut self, time: i32) -> bool {
        if self.min != time {
            self.min = time;
            self.count = 0;
        }
        self.count += 1;
        proxy_wasm::hostcalls::log(
            LogLevel::Debug,
            format!("Obj {:?} {:?}", self.count, self.rpm).as_str(),
        ).ok();
        if let Some(sm) = self.rpm {
            if self.count > sm {
                return false;
            }
        }
        return true;
    }
}
