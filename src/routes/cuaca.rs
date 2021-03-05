use bmkg_wrapper::cuaca::{self, Data, Url};
use bmkg_wrapper::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;
// use serde_json::{json, Value};
use tokio;

#[get("/cuaca/<data>")]
#[tokio::main]
pub async fn cuaca_data(data: String) -> Result<Json<Data>, Status> {
    match Url::from_str(data) {
        Some(url) => {
            let data = cuaca::get_data(url).await;
            match data {
                Ok(val) => Ok(Json(val)),
                _ => Err(Status::InternalServerError),
            }
        }
        None => Err(Status::NotFound),
    }
}

#[get("/cuaca")]
#[tokio::main]
pub async fn cuaca() -> Result<Json<Data>, Error> {
    let data = cuaca::get_data(Url::Indonesia).await?;
    Ok(Json(data))
}
