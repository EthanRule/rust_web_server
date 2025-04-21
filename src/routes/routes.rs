use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use hyper::body::{Body, Bytes};
use hyper::{Method, Request, Response, StatusCode};
mod methods;

pub async fn handle_request(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method()) {
        &Method::CONNECT => methods::connect(req),
        &Method::DELETE => methods::delete(req),
        &Method::GET => methods::get(req),
        &Method::HEAD => methods::head(req),
        &Method::OPTIONS => methods::options(req),
        &Method::PATCH => methods::patch(req),
        &Method::POST => methods::post(req),
        &Method::PUT => methods::put(req),
        &Method::TRACE => methods::trace(req),
        _ => Err("Invalid request method."),
    };
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new().map_err(|never| match never {}).boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into()).map_err(|never| match never {}).boxed()
}
