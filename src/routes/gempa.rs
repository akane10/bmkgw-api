use crate::error::Error;
use bmkgw::gempa::{self, Gempa, Url};
use redis;
use redis::Commands;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio;

fn conn_redis() -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    Ok(con)
}

#[get("/gempa/<data>")]
#[tokio::main]
pub async fn gempa_data(data: String) -> Result<Json<Vec<Gempa>>, Status> {
    match Url::from_str(data) {
        Some(url) => {
            let data = gempa::get_data(url).await;
            match data {
                Ok(val) => Ok(Json(val)),
                _ => Err(Status::InternalServerError),
            }
        }
        None => Err(Status::NotFound),
    }
}

#[get("/gempa")]
#[tokio::main]
pub async fn gempa() -> Result<Json<Vec<Gempa>>, Error> {
    let data = gempa::get_data(Url::Autogempa).await?;
    Ok(Json(data))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sub {
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
}

#[post("/gempa/notif", data = "<sub>")]
pub fn gempa_notif(sub: Json<Sub>) -> Result<Status, redis::RedisError> {
    let mut con = conn_redis()?;
    let auth = sub.auth.clone();
    let data: String = json!(*sub).to_string();
    let _: () = con.set(auth, data)?;

    Ok(Status::Ok)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAuth {
    pub auth: Option<String>,
}
#[delete("/gempa/notif", data = "<sub_auth>")]
pub fn gempa_delete_notif(sub_auth: Json<SubAuth>) -> Result<Status, redis::RedisError> {
    match &sub_auth.auth {
        Some(v) => {
            let mut con = conn_redis()?;
            let _: () = con.del(v)?;

            Ok(Status::Ok)
        }
        _ => Ok(Status::BadRequest),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Res {
    pub key: Option<String>,
}
#[get("/gempa/pub_key")]
pub fn gempa_key() -> Result<Json<Res>, redis::RedisError> {
    let mut con = conn_redis()?;
    let k = con.get("public_key");

    match k {
        Ok(v) => Ok(Json(Res { key: Some(v) })),
        _ => Ok(Json(Res { key: None })),
    }
}
