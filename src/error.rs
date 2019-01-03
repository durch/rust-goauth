use std;
use serde_json::Error as json_error;
use std::io::Error as io_error;
//use curl::Error as curl_error;
use smpl_jwt::error::JwtErr as jwt_error;
use reqwest::Error as reqwest_error;
use auth::TokenErr as token_error;

macro_rules! impl_from {
    ($type_: ident, $enum_ty: ident) => {
        impl From<$type_> for GOErr {
            fn from(e: $type_) -> GOErr {
                GOErr::$enum_ty(e)
            }
        }
    }
}

#[derive(Debug)]
pub enum GOErr {
  Json(json_error),
  Io(io_error),
//  Curl(curl_error),
  JWT(jwt_error),
  Re(reqwest_error),
  TokenErr(token_error),
  Unknown
}

impl_from!(json_error, Json);
impl_from!(io_error, Io);
//impl_from!(curl_error, Curl);
impl_from!(reqwest_error, Re);
impl_from!(jwt_error, JWT);
impl_from!(token_error, TokenErr);

impl std::fmt::Display for GOErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            GOErr::Json(ref e) => e.fmt(f),
            GOErr::Io(ref e) => e.fmt(f),
//            GOErr::Curl(ref e) => e.fmt(f),
            GOErr::JWT(ref e) => e.fmt(f),
            GOErr::TokenErr(ref e) => e.fmt(f),
            GOErr::Re(ref e) => e.fmt(f),
            GOErr::Unknown => write!(f, "An unknown error has occured"),
        }
    }
}

impl std::error::Error for GOErr {
    fn description(&self) -> &str {
        match *self {
            GOErr::Json(ref e) => e.description(),
            GOErr::Io(ref e) => e.description(),
//            GOErr::Curl(ref e) => e.description(),
            GOErr::JWT(ref e) => e.description(),
            GOErr::TokenErr(ref e) => e.description(),
            GOErr::Re(ref e) => e.description(),
            GOErr::Unknown => "unknown error",
        }
    }
}