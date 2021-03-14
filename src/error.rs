use rocket::response::{self, Responder};
use rocket::{http::Status, request::Request};
use std::fmt;

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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::RedisError(ref x) => write!(f, "{:#?}", x.detail()),
            Error::BmkgwError(ref x) => write!(f, "{}", x),
            Error::StatusError(ref x) => write!(f, "{}", x),
            Error::ArgonError(ref x) => write!(f, "{}", x),
            Error::EnvError(ref x) => write!(f, "{}", x),
        }
    }
}

impl std::error::Error for Error {}

macro_rules! error_wrap {
    ($f:ty, $e:expr) => {
        impl From<$f> for Error {
            fn from(f: $f) -> Error {
                $e(f)
            }
        }
    };
}

error_wrap!(redis::RedisError, Error::RedisError);
error_wrap!(bmkgw::Error, Error::BmkgwError);
error_wrap!(Status, Error::StatusError);
error_wrap!(argon2::Error, Error::ArgonError);
error_wrap!(std::env::VarError, Error::EnvError);
