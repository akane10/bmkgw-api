use rocket::response::{self, Responder};
use rocket::{http::Status, request::Request};

#[derive(Debug)]
pub enum Error {
    RedisError(redis::RedisError),
    BmkgwError(bmkgw::Error),
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        println!("Oppss: {:#?}", self);
        Err(Status::InternalServerError)
    }
}

impl From<redis::RedisError> for Error {
    fn from(error: redis::RedisError) -> Self {
        Error::RedisError(error)
    }
}

impl From<bmkgw::Error> for Error {
    fn from(error: bmkgw::Error) -> Self {
        Error::BmkgwError(error)
    }
}
