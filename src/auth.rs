use serde_json;
use time;

use std::*;
use std::error::Error;

use error::GOErr;
use scopes::Scope;

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtClaims {
  iss: String,
  scope: String,
  aud: String,
  exp: i64,
  iat: i64
}

impl JwtClaims {
  pub fn new(service_acc_id: String,
         scope: Scope,
         aud_url: String,
         valid_from: Option<i64>,
         expires_after: Option<i64>) -> Self {
    let iat = match valid_from {
      Some(x) => x,
      None => time::now_utc().to_timespec().sec
    };
    let exp = match expires_after {
      Some(x) => iat + x,
      None => iat + 3600
    };
    JwtClaims {
      iss: service_acc_id,
      scope: scope.url(),
      aud : aud_url,
      exp : exp,
      iat : iat
    }
  }
}

impl fmt::Display for JwtClaims {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", &serde_json::to_string_pretty(&self).unwrap())
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenErr {
  error: String,
  error_description: String
}

impl fmt::Display for TokenErr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "TokenErr: {}", self.error_description)
  }
}

impl Error for TokenErr {
  fn description(&self) -> &str {
    &self.error_description
  }

  fn cause(&self) -> Option<&Error> {
    None
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
  access_token: String,
  token_type: String,
  expires_in: u32
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Token: (type: {}, expires_in: {})", self.token_type, self.expires_in)
  }
}

impl Token {
  pub fn access_token(&self) -> &str {
      &self.access_token
  }

    pub fn token_type(&self) -> &str {
      &self.token_type
    }

  pub fn from_str(response: &str) -> Result<Token, GOErr> {
    match serde_json::from_str(response) {
      Ok(x) => Ok(x),
      Err(e) => {
        match serde_json::from_str(response) {
          Ok(x) => Err(GOErr::TokenErr(x)),
          _ => panic!("{}", e)
        }
      }
    }
  }
}

