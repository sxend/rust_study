use regex::{Captures, Regex};
use std::collections::HashMap;
use std::ops::Index;

fn main() {}

/// nano template engine (https://github.com/trix/nano)
/// # Example
/// use
pub fn nano(template: &str, nanodata: NanoData) -> String {
    Regex::new(r"\{([\w\.]*)\}")
        .unwrap()
        .replace_all(template, move |cap: &Captures| {
            nanodata.get(cap.index(1))
        })
        .to_string()
}

#[derive(Debug, Clone, Default)]
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
    pub fn get(&self, key: &str) -> String {
        self.get_by_keys(&NanoData::split_key(key))
    }
    pub fn put(&mut self, key: &str, value: String) {
        self.put_by_keys(&NanoData::split_key(key), value);
    }
    fn get_by_keys(&self, keys: &[String]) -> String {
        if keys.len() == 1 {
            self.underlying[keys.index(0)].to_owned()
        } else {
            self.children[keys.index(0)]
                .get_by_keys(&keys.split_first().unwrap().1.to_vec()).to_owned()
        }
    }
    fn put_by_keys(&mut self, keys: &[String], value: String) {
        if keys.len() == 1 {
            self.underlying.insert(keys.index(0).to_owned(), value);
        } else {
            self.children
                .entry(keys.index(0).to_owned())
                .or_insert_with(crate::NanoData::new)
                .put_by_keys(&keys.split_first().unwrap().1.to_vec(), value);
        }
    }
    pub fn get_data(&self, key: &str) -> NanoData {
        self.get_data_by_keys(&NanoData::split_key(key))
    }
    pub fn put_data(&mut self, key: &str, value: NanoData) {
        self.put_data_by_keys(&NanoData::split_key(key), value);
    }
    fn get_data_by_keys(&self, keys: &[String]) -> NanoData {
        let mut borrowed = self.children[keys.index(0)].clone();
        if keys.len() != 1 {
            borrowed = borrowed.get_data_by_keys(&keys.split_first().unwrap().1.to_vec())
        }
        borrowed
    }
    fn put_data_by_keys(&mut self, keys: &[String], value: NanoData) {
        if keys.len() == 1 {
            self.children.insert(keys.index(0).to_owned(), value);
        } else {
            self.children
                .entry(keys.index(0).to_owned())
                .or_insert_with(crate::NanoData::new)
                .put_data_by_keys(&keys.split_first().unwrap().1.to_vec(), value);
        }
    }
    fn split_key(key: &str) -> Vec<String> {
        key.split('.').map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn with_put() {
        let template = "\
=======
a => {a}
a.b => {a.b}
a.b.c => {a.b.c}
=======
    ";
        let mut data = crate::NanoData::new();
        data.put("a", "a value".to_string());
        data.put("a.b", "a.b value".to_string());
        data.put("a.b.c", "a.b.c value".to_string());
        println!("{}", crate::nano(template, data));
    }

    #[test]
    fn with_put_data() {
        let template = "\
=======
a => {a}
a.b => {a.b}
a.b.c => {a.b.c}
=======
    ";
        let mut data = crate::NanoData::new();
        data.put("a", "a value".to_string());
        let mut a = crate::NanoData::new();
        a.put("b", "a.b value".to_string());
        let mut b = crate::NanoData::new();
        b.put("c", "a.b.c value".to_string());
        data.put_data("a", a);
        data.put_data("a.b", b);
        println!("{}", crate::nano(template, data));
    }
}
