use bmkg_wrapper::gempa::{self, Url};
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use std::error::Error;

#[get("/gempa/<data>")]
pub fn gempa_data(data: String) -> Result<Json<Value>, Status> {
    match Url::from_str(data) {
        Some(url) => {
            let data = gempa::get_data(url).and_then(gempa::to_json);
            match data {
                Ok(val) => Ok(Json(json!({ "data": val }))),
                _ => Err(Status::InternalServerError),
            }
        }
        None => Err(Status::NotFound),
    }
}

#[get("/gempa")]
pub fn gempa() -> Result<Json<Value>, Box<dyn Error>> {
    let data = gempa::get_data(Url::Autogempa).and_then(gempa::to_json);

    match data {
        Ok(val) => Ok(Json(json!({ "data": val }))),
        Err(e) => Err(e),
    }
}
