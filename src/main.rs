#[macro_use]
extern crate rocket;
// ===========
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::{
    get,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_okapi::okapi::schemars;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes};
use schemars::JsonSchema;

// ===========
#[openapi(tag = "Users")]
#[get("/getData")]
fn index() -> &'static str {
    "Hello, world!"
}

fn get_docs() -> SwaggerUIConfig {
    // use rocket_okapi::settings::UrlObject;

    SwaggerUIConfig {
        url: "../openapi.json".to_string(),
        ..Default::default()
    }
}

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
        .mount("/", openapi_get_routes![index])
        .mount("/api", make_swagger_ui(&get_docs()))
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}
