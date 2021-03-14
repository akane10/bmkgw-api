use rocket::response::{self, Responder};
use rocket::{http::Status, request::Request};

#[derive(Debug)]
pub enum Error {
    RedisError(redis::RedisError),
    BmkgwError(bmkgw::Error),
    ArgonError(argon2::Error),
    StatusError(Status),
    EnvError(std::env::VarError),
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        println!("Oppss: {:#?}", self);
        match self {
            Error::StatusError(e) => Err(e),
            _ => Err(Status::InternalServerError),
        }
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

impl From<Status> for Error {
    fn from(error: Status) -> Self {
        Error::StatusError(error)
    }
}

impl From<argon2::Error> for Error {
    fn from(error: argon2::Error) -> Self {
        Error::ArgonError(error)
    }
}

impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Self {
        Error::EnvError(error)
    }
}
