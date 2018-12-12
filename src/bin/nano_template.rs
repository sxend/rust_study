use regex::{Captures, Regex};
use std::collections::HashMap;
use std::ops::Index;

fn main() {}

/// nano template engine (https://github.com/trix/nano)
/// # Example
/// use
pub fn nano(template: String, nanodata: NanoData) -> String {
    Regex::new(r"\{([\w\.]*)\}")
        .unwrap()
        .replace_all(template.as_str(), move |cap: &Captures| {
            nanodata.get(cap.index(1).to_string())
        })
        .to_string()
}

#[derive(Debug, Clone)]
pub struct NanoData {
    underlying: HashMap<String, String>,
    children: HashMap<String, NanoData>,
}

impl NanoData {
    pub fn new() -> NanoData {
        NanoData {
            underlying: HashMap::new(),
            children: HashMap::new(),
        }
    }
    pub fn get(&self, key: String) -> String {
        self.get_by_keys(&NanoData::split_key(key))
    }
    pub fn put(&mut self, key: String, value: String) {
        self.put_by_keys(&NanoData::split_key(key), value);
    }
    fn get_by_keys(&self, keys: &Vec<String>) -> String {
        if keys.len() == 1 {
            self.underlying.get(keys.index(0)).unwrap().to_string()
        } else {
            self.children
                .get(keys.index(0))
                .unwrap()
                .get_by_keys(&keys.split_first().unwrap().1.to_vec())
        }
    }
    fn put_by_keys(&mut self, keys: &Vec<String>, value: String) {
        if keys.len() == 1 {
            self.underlying.insert(keys.index(0).to_owned(), value);
        } else {
            self.children
                .entry(keys.index(0).to_owned())
                .or_insert(crate::NanoData::new())
                .put_by_keys(&keys.split_first().unwrap().1.to_vec(), value);
        }
    }
    pub fn get_data(&self, key: String) -> NanoData {
        self.get_data_by_keys(&NanoData::split_key(key))
    }
    pub fn put_data(&mut self, key: String, value: NanoData) {
        self.put_data_by_keys(&NanoData::split_key(key), value);
    }
    fn get_data_by_keys(&self, keys: &Vec<String>) -> NanoData {
        let mut borrowed = self.children.get(keys.index(0)).unwrap().clone();
        if keys.len() != 1 {
            borrowed = borrowed.get_data_by_keys(&keys.split_first().unwrap().1.to_vec())
        }
        borrowed
    }
    fn put_data_by_keys(&mut self, keys: &Vec<String>, value: NanoData) {
        if keys.len() == 1 {
            self.children.insert(keys.index(0).to_owned(), value);
        } else {
            self.children
                .entry(keys.index(0).to_owned())
                .or_insert(crate::NanoData::new())
                .put_data_by_keys(&keys.split_first().unwrap().1.to_vec(), value);
        }
    }
    fn split_key(key: String) -> Vec<String> {
        key.split(".").map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn with_put() {
        let template: String = "\
=======
a => {a}
a.b => {a.b}
a.b.c => {a.b.c}
=======
    "
            .to_string();
        let mut data = crate::NanoData::new();
        data.put("a".to_string(), "a value".to_string());
        data.put("a.b".to_string(), "a.b value".to_string());
        data.put("a.b.c".to_string(), "a.b.c value".to_string());
        println!("{}", crate::nano(template, data));
    }

    #[test]
    fn with_put_data() {
        let template: String = "\
=======
a => {a}
a.b => {a.b}
a.b.c => {a.b.c}
=======
    "
            .to_string();
        let mut data = crate::NanoData::new();
        data.put("a".to_string(), "a value".to_string());
        let mut a = crate::NanoData::new();
        a.put("b".to_string(), "a.b value".to_string());
        let mut b = crate::NanoData::new();
        b.put("c".to_string(), "a.b.c value".to_string());
        data.put_data("a".to_string(), a);
        data.put_data("a.b".to_string(), b);
        println!("{}", crate::nano(template, data));
    }
}
