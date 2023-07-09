use jq_rs;

#[macro_use]
extern crate rocket;

#[get("/jq/<query>/<json>")]
fn jq(query: String, json: String) -> Result<String, String> {
    jq_rs::run(&query, &json).map_err(|e| e.to_string())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, jq]);

    Ok(rocket.into())
}
