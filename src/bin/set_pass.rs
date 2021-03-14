use argon2::{self, Config};
use bmkgw_api::error::Error;
use dotenv;
use redis;
use redis::Commands;
use std::env;

fn conn_redis() -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    Ok(con)
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let secret = env::var("SECRET_KEY")?;
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let config = Config::default();
        let hash = argon2::hash_encoded(args[1].as_bytes(), secret.as_bytes(), &config)?;
        conn_redis()?.set("pass_u", hash)?;
        println!("done");
        Ok(())
    } else {
        println!("missing password argument");
        Ok(())
    }
}
