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
            .get_mut(&key.to_string())
            .unwrap()
            .push(value.to_string())
    }

    pub fn Set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), vec![value.to_string()]);
    }

    pub fn Get(&self, key: &str) -> String {
        self.0
            .get(key)
            .unwrap_or(&vec!["".to_string()])
            .get(0)
            .unwrap()
            .to_string()
    }
}
