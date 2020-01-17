use serde_json;
use time;

use std::*;
use std::error::Error;
use std::str::FromStr;

use error::GOErr;
use scopes::Scope;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtClaims {
    iss: String,
    scope: String,
    aud: String,
    exp: i64,
    iat: i64
}

impl JwtClaims {
    pub fn new(service_acc_id: String,
               scope: &Scope,
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
            aud: aud_url,
            exp,
            iat
        }
    }
}

impl fmt::Display for JwtClaims {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &serde_json::to_string_pretty(&self).unwrap())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    
    pub fn expires_in(&self) -> u32 {
        self.expires_in
    }
}

impl FromStr for Token {
    type Err = GOErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str(s) {
            Ok(x) => Ok(x),
            Err(e) => {
                match serde_json::from_str(s) {
                    Ok(x) => Err(GOErr::TokenErr(x)),
                    _ => panic!("{}", e)
                }
            }
        }
    }
}

