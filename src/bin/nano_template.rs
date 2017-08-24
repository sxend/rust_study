extern crate regex;

use regex::{Regex, Captures};
use std::collections::HashMap;
use std::ops::Index;
use std::rc::*;
use std::cell::*;


fn main() {
}

/// nano template engine (https://github.com/trix/nano)
/// # Example
/// use
pub fn nano(template: String, nanodata: NanoData) -> String {
    Regex::new(r"\{([\w\.]*)\}").unwrap().replace_all(template.as_str(), move |cap: &Captures| {
        nanodata.get(cap.index(1).to_string())
    }).to_string()
}

type HashRef<T> = Rc<RefCell<HashMap<String, T>>>;

#[derive(Debug, Clone)]
pub struct NanoData {
    underlying: HashRef<String>,
    children: HashRef<NanoData>
}

impl NanoData {
    pub fn new() -> NanoData {
        NanoData {
            underlying: Rc::new(RefCell::new(HashMap::new())),
            children: Rc::new(RefCell::new(HashMap::new()))
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
            self.underlying.borrow().get(keys.index(0)).unwrap().to_string()
        } else {
            self.children.borrow().get(keys.index(0)).unwrap().get_by_keys(&keys.split_first().unwrap().1.to_vec())
        }
    }
    fn put_by_keys(&mut self, keys: &Vec<String>, value: String) {
        if keys.len() == 1 {
            self.underlying.borrow_mut().insert(keys.index(0).to_owned(), value);
        } else {
            self.children.borrow_mut()
                .entry(keys.index(0).to_owned()).or_insert(NanoData::new())
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
        let mut borrowed = (*self.children).borrow().get(keys.index(0)).unwrap().clone();
        if keys.len() != 1 {
            borrowed = borrowed.get_data_by_keys(&keys.split_first().unwrap().1.to_vec())
        }
        borrowed
    }
    fn put_data_by_keys(&mut self, keys: &Vec<String>, value: NanoData) {
        if keys.len() == 1 {
            self.children.borrow_mut().insert(keys.index(0).to_owned(), value);
        } else {
            self.children.borrow_mut()
                .entry(keys.index(0).to_owned()).or_insert(NanoData::new())
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
    fn it_works() {
        let template: String = "\
    hoge
        fuga
    [{string_data}]
    [{nested}]
    [{nested.data0}]
    [{nested.data1}]
    [{foo.data.put_data}]
    [{foo}]
        fizz
    ".to_string();
        let mut data = ::NanoData::new();
        data.put("string_data".to_string(), "string data".to_string());
        data.put("nested".to_string(), "nested value".to_string());
        data.put("nested.data0".to_string(), "nested data0".to_string());
        data.put("nested.data1".to_string(), "nested data1".to_string());
        data.put("foo".to_string(), "foo value".to_string());
        let mut put_data = ::NanoData::new();
        put_data.put("put_data".to_string(), "put value".to_string());
        data.put_data("foo.data".to_string(), put_data);
        println!("{}", ::nano(template, data));
    }
}
