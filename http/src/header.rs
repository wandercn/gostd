#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::collections::HashMap;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Header(pub HashMap<String, Vec<String>>);

impl Header {
    pub fn NewWithHashMap(m: HashMap<String, Vec<String>>) -> Header {
        Header(m)
    }
    pub fn Add(&mut self, key: &str, value: &str) {
        self.0
            .entry(key.to_string())
            .or_insert_with(Vec::new)
            .push(value.to_string())
    }

    pub fn Set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), vec![value.to_string()]);
    }

    pub fn Get(&self, key: &str) -> &str {
        self.0
            .get(key)
            .and_then(|v| v.get(0))
            .map(|s| s.as_str())
            .unwrap_or("")
    }
}
