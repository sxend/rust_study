use actix_web;
use actix_web::*;
use env_logger;
use futures::future;
use serde;
use serde_json;

fn main() {
    env_logger::init();
    actix_web::server::new(|| actix_web::App::new().resource("/", |r| r.with_async(handle)))
        .bind("0.0.0.0:3000")
        .expect("can not bind 3000")
        .run()
}

fn handle(_req: HttpRequest) -> Box<future::Future<Item = HttpResponse, Error = error::Error>> {
    let mut hashmap = std::collections::HashMap::new();
    hashmap.insert(rand::random::<u64>(), uuid::Uuid::new_v4());
    Box::from(future::ok(HttpResponse::from(
        serde_json::to_string_pretty(&hashmap),
    )))
}
