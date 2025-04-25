use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use hyper::body::{Body, Bytes};
use hyper::{Method, Request, Response, StatusCode}; 

pub async fn handle_request(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method()) {
        &Method::CONNECT => connect(req),
        &Method::DELETE => delete(req),
        &Method::GET => get(req),
        &Method::HEAD => head(req),
        &Method::OPTIONS => options(req),
        &Method::PATCH => patch(req),
        &Method::POST => post(req),
        &Method::PUT => put(req),
        &Method::TRACE => trace(req),
        _ => Err("Invalid request method."),
    };
}


fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new().map_err(|never| match never {}).boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into()).map_err(|never| match never {}).boxed()
}
