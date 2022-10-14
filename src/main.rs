#[macro_use]
extern crate rocket;

// ===========
use chrono::Datelike;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::{
    get,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_okapi::okapi::schemars;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes};
use schemars::JsonSchema;
// use validator::{Validate, ValidationError};
use rocket_validation::{Validate, Validated};

// ======Root service=====
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ResponseI {
    error: bool,
    status: i32,
    message: String,
    response: Option<String>,
}

#[openapi(tag = "Servicio")]
#[get("/")]
fn index() -> Option<Json<ResponseI>> {
    let year = chrono::Utc::now().year();

    Some(Json(ResponseI {
        error: false,
        status: 200,
        message: format!("Bienvenido al Servicio de Almacenamiento de Archivos, Basado en Principios REST, Copyright © Ministerio Público {year}"),
        response: None
    }))
}

// ========service for save files=============
// #[derive(Serialize, Deserialize, JsonSchema)]
#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct InputData {
    #[validate(length(min = 1))]
    appName: String,
    nas: String,
    path: String,
    fileName: u16,
    #[validate(range(min = 0, max = 10))]
    age: u8,
    base64: String,
    withDateSubFolder: bool,
}

#[openapi(tag = "Archivo")]
#[post("/write/base64", data = "<input>")]
fn write_base64(input: Json<InputData>) -> Option<Json<ResponseI>> {
    let year = chrono::Utc::now().year();

    Some(Json(ResponseI {
        error: false,
        status: 200,
        message: format!("Bienvenido al Servicio de Almacenamiento de Archivos, Basado en Principios REST, Copyright © Ministerio Público {year}"),
        response: None
    }))
}

// ========validation
#[derive(Debug, Deserialize, Serialize, Validate)]
///  Implements `Validate`
#[serde(crate = "rocket::serde")]
pub struct HelloData {
    #[validate(length(min = 1))]
    ///  Your validation annotation
    name: String,
    #[validate(range(min = 0, max = 100))]
    ///  Your validation annotation
    age: u8,
}

#[post("/hello", format = "application/json", data = "<data>")]
fn validated_hello(
    data: /* Uses the `Validated` type */ Validated<Json<HelloData>>,
) -> Json<HelloData> {
    Json(data.0 .0)
}

// ===========swagger config method
fn get_docs() -> SwaggerUIConfig {
    // use rocket_okapi::settings::UrlObject;

    SwaggerUIConfig {
        url: "../openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", openapi_get_routes![index, write_base64])
        .mount("/api", make_swagger_ui(&get_docs()))
        .mount("/", routes![validated_hello])
        .register("/", catchers![rocket_validation::validation_catcher])
}

// match input_data.validate() {
//     Ok(_) => (),
//     Err(e) => return e;
// };
