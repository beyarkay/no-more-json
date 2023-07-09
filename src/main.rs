use jq_rs;

#[macro_use]
extern crate rocket;

#[get("/<json>?<query>")]
fn jq(query: String, json: String) -> Result<String, String> {
    jq_rs::run(&query, &json).map_err(|e| e.to_string())
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![jq]);

    Ok(rocket.into())
}
