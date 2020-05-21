use serde::{Serialize,Deserialize};
use proxy_wasm; 
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct RateLimiter {
    RPM: Option<u32>,
    Min: i32,
    Count: u32,
    key: String,
}

impl RateLimiter {
    fn new(key: &String, plan: &String) -> Self {
        let limit = match plan.as_str() {
            "Team" => Some(800),
            "Enterprise" => None,
            _ => Some(400),
        };
        Self {
            RPM: limit,
            Min: -1,
            Count: 0,
            key: key.clone()
        }
    }
    pub fn get(key: String, plan: String) -> Self {
        if let Ok(data) = proxy_wasm::hostcalls::get_shared_data(&key.clone()) {
            if let Some(data) = data.0 {
                let data: Option<Self> = bincode::deserialize(&data).unwrap_or(None);
                if let Some(obj) = data {
                    return obj
                }
            }
        }
        return Self::new(&key, &plan)
    }
    pub fn set(&self) {
        let target: Option<Self>  = Some(self.clone());
        let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
        proxy_wasm::hostcalls::set_shared_data(&self.key.clone(), Some(&encoded), None);
    }
    pub fn update(&mut self, time: i32) -> bool {
        if self.Min != time {
            self.Min = time;
            self.Count = 0;
        }
        self.Count += 1;
        if let Some(sm) = self.RPM {
            if self.Count > sm {
                return false
            }
        }
        proxy_wasm::hostcalls::log(LogLevel::Debug, format!("Obj {:?}", self.Count).as_str());
        true
    }
}