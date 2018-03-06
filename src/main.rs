#[macro_use] extern crate hyper;
extern crate iron;
#[macro_use] extern crate lazy_static;
extern crate params;
extern crate router;
#[macro_use] extern crate serde_derive;

use std::io::Read;

use iron::status;
use iron::headers::ContentType;
use iron::prelude::*;
use params::Params;
use router::Router;

mod http_util;
mod qiita;

fn items(req: &mut Request) -> IronResult<Response> {

    let queries = req.get::<Params>().unwrap();
    let page = match http_util::safe_find(&queries, &"page".to_string()) {
        Some(v) => { v },
        None => { "10" }
    };
    let per_page = match http_util::safe_find(&queries, &"per_page".to_string()) {
        Some(v) => { v },
        None => { "10" }
    };

    let query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");

    let url = qiita::get_items_url(query, page, per_page);

    let client = http_util::generate_client();
    let header = http_util::qiita_header();
    let mut res = client.get(&url)
        .headers(header).send().unwrap();
    let mut buf = String::new();
    res.read_to_string(&mut buf).unwrap();
    Ok(Response::with((ContentType::json().0, status::Ok, buf)))
}

fn tags(req: &mut Request) -> IronResult<Response> {
    let url = qiita::get_tags_url();

    let client = http_util::generate_client();
    let mut res = client.get(&url).send().unwrap();
    let mut buf = String::new();
    res.read_to_string(&mut buf).unwrap();
    Ok(Response::with((ContentType::json().0, status::Ok, buf)))
}

fn handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

fn bad(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::BadRequest))
}


fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", tags, "tags");
    router.get("/:query/items/", items, "items");
    let host = format!("{}:{}", http_util::CONFIG.server.ip, http_util::CONFIG.server.port);
    Iron::new(router).http(host).unwrap();
}

