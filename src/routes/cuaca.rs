use bmkg_wrapper::cuaca::{self, Data, Domain, Url};
use bmkg_wrapper::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;
// use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub url_param: String,
}

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

#[get("/cuaca/location")]
pub fn location() -> Result<Json<Vec<Location>>, Error> {
    let data: Vec<Location> = Domain::get_data()
        .into_iter()
        .map(|x| Location {
            name: x.name,
            url_param: x.value,
        })
        .collect();
    Ok(Json(data))
}
