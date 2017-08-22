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
pub fn nano(template: String, data: HashMap<String, String>) -> String {
    unimplemented!()
}

pub fn nano_with_data(template: String, nanodata: NanoData) -> String {
    Regex::new(r"\{([\w\.]*)\}").unwrap().replace_all(template.as_str(), move |cap: &Captures| {
        nanodata.get(cap.index(1).to_string())
    }).to_string()
}

type HashRef<T> = Rc<RefCell<HashMap<String, T>>>;

#[derive(Debug)]
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
        let key_chain = key.split(".").map(|s| s.to_string()).collect();
        self.get_with_keys(&key_chain)
    }
    pub fn put(&mut self, key: String, value: String) {
        let key_chain = key.split(".").map(|s| s.to_string()).collect();
        self.put_with_keys(&key_chain, value);
    }
    fn get_with_keys(&self, keys: &Vec<String>) -> String {
        if keys.len() == 1 {
            (*self.underlying).borrow().get(keys.index(0)).unwrap().to_string()
        } else {
            (*self.children).borrow().get(keys.index(0)).unwrap().get_with_keys(&keys.split_first().unwrap().1.to_vec())
        }
    }
    fn put_with_keys(&mut self, keys: &Vec<String>, value: String) {
        if keys.len() == 1 {
            (*self.underlying).borrow_mut().insert(keys.index(0).to_owned(), value);
        } else {
            let key_chain = keys.split_first().unwrap().1.to_vec();
            if let Some(_) = (*self.children).borrow().get(keys.index(0)) {
                let &mut child = &mut (*self.children).borrow().get(keys.index(0)).unwrap();
                child.put_with_keys(&key_chain, value);
            } else {
                let mut child = NanoData::new();
                child.put_with_keys(&key_chain, value);
                (*self.children).borrow_mut().insert(keys.index(0).to_owned(), child);
            }
        }
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
    [{nested.data0}]
    [{nested.data1}]
        fizz
    ".to_string();
        let mut data = ::NanoData::new();
        data.put("string_data".to_string(), "string data".to_string());
        data.put("nested.data0".to_string(), "nested data0".to_string());
        data.put("nested.data1".to_string(), "nested data1".to_string());
        ::nano_with_data(template, data);
    }
}
