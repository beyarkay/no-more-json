use jq_rs;
use shuttle_runtime::tracing::info;

#[macro_use]
extern crate rocket;

#[get("/api/<url..>?<q>")]
fn jqapi(url: Option<String>, q: String) -> Result<String, String> {
    info!("Got request /api/{url:?}?q={q}");
    if let Some(url) = url {
        let json = reqwest::blocking::get(url)
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())?;
        do_jq(json, q)
    } else {
        Err("URL is required".to_string())
    }
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
