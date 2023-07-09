//! no-more-json
//!
//! Fed up with JSON APIs? Just want one little value from a massive JSON object? Don't want to
//! parse the JSON yourself?
//!
//! `no-more-json` does all this for you! Here's an example:
//!
//! Before 😓:
//! ```
//! curl https://httpbin.org/ip
//! {
//!   "origin": "102.222.181.247"
//! }
//! ```
//!
//! After:
//! ```
//! curl 'https://jq-ify.shuttleapp.rs/api?url=https://httpbin.org/ip&q=.origin'
//! "13.41.13.143"
//! ```
//!
//! Let's break that down:
//!
//!```
//!   https://jq-ify.shuttleapp.rs/api  <-- This is the API for no-more-json
//!      ?url=https://httpbin.org/ip    <-- This is where you specify the url for your endpoint
//!      &q=.origin                     <-- This is where you put your jq query to parse the JSON
//!```
//! If you don't know about `jq`, check this out: https://jqlang.github.io/jq/
//!
//! So this lets you parse arbitrary JSON blobs and just get back a simple scalar value, without
//! having to parse the JSON yourself, include a json-parsing library, or trying to include the
//! `jq` binary into your service.
use jq_rs;
use shuttle_runtime::tracing::info;

#[macro_use]
extern crate rocket;

#[get("/api?<q>&<url>")]
fn jqapi(q: String, url: String) -> Result<String, String> {
    info!("Got request /api?q={q}&url={url}");
    let json = reqwest::blocking::get(url)
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?;
    do_jq(json, q)
}

#[get("/<json>?<q>")]
fn jq(json: String, q: String) -> Result<String, String> {
    info!("Got request /{json}?q={q}");
    do_jq(json, q)
}

fn do_jq(json: String, q: String) -> Result<String, String> {
    info!("Running jq on {json} with {q}");
    jq_rs::run(&q, &json).map_err(|e| e.to_string())
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![jq, jqapi]);

    Ok(rocket.into())
}
