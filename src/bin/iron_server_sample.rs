extern crate futures;
extern crate iron;
extern crate router;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate uuid;

use futures::*;
use futures::future::*;
use std::error::Error;
use std::convert::From;
use iron::*;
use iron::typemap;
use iron::headers::*;
use router::Router;
use uuid::Uuid;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "GET /");
    router.get("/future", future_handler, "GET /future");
    let mut chain = Chain::new(router);
    chain.link_before(assign_request_id);
    chain.link_before(authentication_filter);
    chain.link_after(start_session);
    let listen_address =
        std::env::var("LISTEN_ADDRESS".to_owned()).unwrap_or("0.0.0.0:3000".to_owned());
    Iron::new(chain).http(listen_address).unwrap();
}

fn future_handler(_: &mut Request) -> IronResult<Response> {
    let future = futures::future::result(Ok(Response::with(
        (status::Ok, "is future response".to_string()),
    )));
    IronResult::from(FutureToIronResult::from(future))
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
        Err(IronError::new(
            StringError("authentication failed.".to_string()),
            status::BadRequest,
        ))
    }
}

fn start_session(req: &mut Request, mut res: Response) -> IronResult<Response> {
    if !req.headers.has::<Cookie>() {
        let cookie = format!(
            "sid={}; Path=/; Domain=0.0.0.0; Max-Age={}",
            gen_uuid(),
            3600
        );
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
            timestamp: current_time_millis(),
        },
        payload: Message {
            message: "hello".to_string(),
        },
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
    fn description(&self) -> &str {
        &*self.0
    }
}

struct FutureToIronResult {
    e: Option<IronError>,
    r: Option<Response>,
}

impl From<FutureToIronResult> for IronResult<Response> {
    fn from(intermediate: FutureToIronResult) -> IronResult<Response> {
        if intermediate.r.is_some() {
            Ok(intermediate.r.unwrap())
        } else {
            Err(intermediate.e.unwrap())
        }
    }
}

impl From<FutureResult<Response, IronError>> for FutureToIronResult {
    fn from(mut result: FutureResult<Response, IronError>) -> Self {
        match result.poll().unwrap() {
            Async::Ready(t) => FutureToIronResult {
                r: Some(t),
                e: None,
            },
            Async::NotReady => FutureToIronResult {
                r: None,
                e: Some(IronError::new(StringError("error".to_string()), "error")),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    request_id: String,
    timestamp: i64,
}

#[derive(Serialize, Deserialize)]
struct ResponseData<A> {
    metadata: Metadata,
    payload: A,
}

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}
