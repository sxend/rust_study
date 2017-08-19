extern crate regex;

use regex::{Regex, Captures};
use std::ops::Index;
use std::collections::HashMap;

fn main() {
    let template = "\
    hoge
        fuga
    [{string_data}]
    [{nested.data}]
        fizz
    ".to_string();
    let mut data: HashMap<String, TemplateValue>  = HashMap::new();
    data.insert("string_data".to_string(), TemplateValue::String("string_data value".to_string()));
    let mut nested: HashMap<String, TemplateValue>  = HashMap::new();
    nested.insert("data".to_string(), TemplateValue::String("nested.data value".to_string()));
    data.insert("nested".to_string(), TemplateValue::Map(nested));
    println!("{}", templating(template, TemplateValue::Map(data)));
}
fn templating(template: String, data: TemplateValue) -> String {
    let re = Regex::new(r"\{([\w\.]*)\}").unwrap();
    let result = re.replace_all(template.as_str(), |cap: &Captures| {
        let mut temp_data: &TemplateValue = &data;
        let mut result_string: &String = &"".to_string();
        let keys = cap.index(1).split(".");
        for key in keys {
            match temp_data {
                &TemplateValue::Map(ref map) => {
                    match map.get(key) {
                        Some(m@&TemplateValue::Map(_)) => {
                            temp_data = m
                        },
                        Some(&TemplateValue::String(ref s)) => {
                            result_string = s;
                            break;
                        }
                        None => panic!("key not found: {}", key)
                    }
                },
                &TemplateValue::String(ref s) => {
                    result_string = s;
                    break;
                }
            }
        }
        result_string.to_owned()
    });
    result.to_string()
}

#[derive(Debug)]
enum TemplateValue {
    String(String),
    Map(HashMap<String, TemplateValue>)
}