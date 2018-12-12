use std::any::Any;

fn main() {
    receiver(&"&str message");
    receiver(&100);
    receiver(&Box::new("Box<&str> message"));
    receiver(&Box::new("Box<String> message ".to_string()));
}

fn receiver(message: &Any) {
    if let Some(message) = message.downcast_ref::<&str>() {
        println!("&str: {}", message)
    }
    if let Some(message) = message.downcast_ref::<i32>() {
        println!("i32: {}", message)
    }
    if let Some(message) = message.downcast_ref::<Box<&str>>() {
        println!("Box<&str>: {}", message)
    }
    if let Some(message) = message.downcast_ref::<Box<String>>() {
        println!("Box<String>: {}", message)
    }
}
