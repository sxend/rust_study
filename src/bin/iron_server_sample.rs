extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate time;
extern crate uuid;

use std::error::Error;
use iron::*;
use iron::typemap;
use iron::headers::*;
use router::Router;
use uuid::Uuid;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "GET /");
    router.get("/foo", handler, "GET /foo");
    let mut chain = Chain::new(router);
    chain.link_before(assign_request_id);
    chain.link_before(authentication_filter);
    chain.link_after(start_session);
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn handler(req: &mut Request) -> IronResult<Response> {
    serde_json::to_string_pretty(&gen_response_data(req))
        .map_err(|err| {
            let description = err.description().to_string();
            IronError::new(err, description)
        })
        .map(|data| {
            Response::with((status::Ok, ContentType::json().0, data))
        })
}

struct RequestId {}

impl typemap::Key for RequestId {
    type Value = String;
}

fn assign_request_id(req: &mut Request) -> IronResult<()> {
    req.extensions.insert::<RequestId>(gen_uuid());
    Ok(())
}

fn authentication_filter(req: &mut Request) -> IronResult<()> {
    if req.url.path().as_slice() == [""] {
        Ok(())
    } else if let Some(_) = req.headers.get::<Cookie>() {
        Ok(())
    } else {
        Err(IronError::new(StringError("authentication failed.".to_string()), status::BadRequest))
    }
}

fn start_session(req: &mut Request, mut res: Response) -> IronResult<Response> {
    if !req.headers.has::<Cookie>() {
        let cookie = format!("sid={}; Path=/; Domain=localhost; Max-Age={}", gen_uuid(), 3600);
        res.headers.set(SetCookie(vec![cookie.to_string()]));
    }
    Ok(res)
}

fn gen_uuid() -> String {
    Uuid::new_v4().hyphenated().to_string()
}

fn gen_response_data(req: &mut Request) -> ResponseData<Message> {
    ResponseData {
        metadata: Metadata {
            request_id: req.extensions.get::<RequestId>().unwrap().to_owned(),
            timestamp: current_time_millis()
        },
        payload: Message {
            message: "hello".to_string()
        }
    }
}

fn current_time_millis() -> i64 {
    let timestamp = time::now_utc().to_timespec();
    (timestamp.sec * 1000) + (timestamp.nsec as i64 / 1000000)
}

#[derive(Debug)]
struct StringError(String);

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Error for StringError {
    fn description(&self) -> &str { &*self.0 }
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    request_id: String,
    timestamp: i64
}

#[derive(Serialize, Deserialize)]
struct ResponseData<A> {
    metadata: Metadata,
    payload: A,
}

#[derive(Serialize, Deserialize)]
struct Message {
    message: String
}