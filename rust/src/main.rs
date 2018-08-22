extern crate actix_web;
extern crate futures;
#[macro_use]
extern crate json;
extern crate reqwest;

use actix_web::{server, App, AsyncResponder, HttpRequest};
use json::{parse, Error as JsonError, JsonValue};
use std::iter::Iterator;
use std::result::Result as StdResult;

fn reddit() -> StdResult<JsonValue, RequestOrJsonError> {
    let url = "https://www.reddit.com/r/politics/hot.json";
    let body = reqwest::get(url)?.text()?;
    Ok(parse(&body[..]).unwrap())
}

fn children(val: &JsonValue) -> impl Iterator<Item = &JsonValue> {
    let children = &val["data"]["children"];
    children.members().map(|l| &l["data"])
}

fn index(
    _req: &HttpRequest,
) -> Box<futures::Future<Item = actix_web::HttpResponse, Error = actix_web::Error>> {
    let resp = reddit().unwrap();
    let mut arr = JsonValue::new_array();
    for listing in children(&resp) {
        let row = object!{
            "title" => listing["title"].clone()
        };
        arr.push(row).unwrap();
    }
    futures::future::result(Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(arr.dump())))
        .responder()
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.route().a(index)))
        .bind("127.0.0.1:3000")
        .unwrap()
        .run();
}

#[derive(Debug)]
enum RequestOrJsonError {
    REQUEST(reqwest::Error),
    JSON(json::Error),
}

impl From<reqwest::Error> for RequestOrJsonError {
    fn from(err: reqwest::Error) -> RequestOrJsonError {
        RequestOrJsonError::REQUEST(err)
    }
}

impl From<JsonError> for RequestOrJsonError {
    fn from(err: JsonError) -> RequestOrJsonError {
        RequestOrJsonError::JSON(err)
    }
}
