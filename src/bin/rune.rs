use iron::*;
use router::Router;
use params::{Params, Value};

extern crate base64;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "GET /");
    let listen_address =
        std::env::var("LISTEN_ADDRESS".to_owned()).unwrap_or("0.0.0.0:3000".to_owned());
    Iron::new(router).http(listen_address).unwrap();
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<Params>().unwrap();
    match map.find(&["q"]) {
        Some(&Value::String(ref q)) => {
            Ok(Response::with((iron::status::Ok, base_rune::from_str(q))))
        }
        _ => Ok(Response::with(iron::status::NotFound)),
    }
}


mod base_rune {
    static CHARS: [&str; 81] = [
        "ᚠ", "ᚡ", "ᚢ", "ᚣ", "ᚤ", "ᚥ", "ᚦ", "ᚧ", "ᚨ", "ᚩ", "ᚪ", "ᚫ", "ᚬ", "ᚭ", "ᚮ", "ᚯ",
        "ᚰ", "ᚱ", "ᚲ", "ᚳ", "ᚴ", "ᚵ", "ᚶ", "ᚷ", "ᚸ", "ᚹ", "ᚺ", "ᚻ", "ᚼ", "ᚽ", "ᚾ", "ᚿ",
        "ᛀ", "ᛁ", "ᛂ", "ᛃ", "ᛄ", "ᛅ", "ᛆ", "ᛇ", "ᛈ", "ᛉ", "ᛊ", "ᛋ", "ᛌ", "ᛍ", "ᛎ", "ᛏ",
        "ᛐ", "ᛑ", "ᛒ", "ᛓ", "ᛔ", "ᛕ", "ᛖ", "ᛗ", "ᛘ", "ᛙ", "ᛚ", "ᛛ", "ᛜ", "ᛝ", "ᛞ", "ᛟ",
        "ᛠ", "ᛡ", "ᛢ", "ᛣ", "ᛤ", "ᛥ", "ᛦ", "ᛧ", "ᛨ", "ᛩ", "ᛪ", "᛫", "᛬", "᛭", "ᛮ", "ᛯ", "ᛰ"
    ];
    pub fn from_str(_str: &String) -> String {
        CHARS.concat().to_string()
    }
}