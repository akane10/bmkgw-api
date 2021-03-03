use bmkg_wrapper::gempa::{self, Url};
use bmkg_wrapper::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use tokio;

#[get("/gempa/<data>")]
#[tokio::main]
pub async fn gempa_data(data: String) -> Result<Json<Value>, Status> {
    match Url::from_str(data) {
        Some(url) => {
            let data = gempa::get_data(url).await;
            match data {
                Ok(val) => Ok(Json(json!({ "data": val }))),
                _ => Err(Status::InternalServerError),
            }
        }
        None => Err(Status::NotFound),
    }
}

#[get("/gempa")]
#[tokio::main]
pub async fn gempa() -> Result<Json<Value>, Error> {
    let data = gempa::get_data(Url::Autogempa).await?;
    Ok(Json(json!({ "data": data })))
}
