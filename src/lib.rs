#![allow(dead_code)]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate time;
extern crate curl;
#[macro_use]
extern crate log;
extern crate smpl_jwt;

pub mod error;
pub mod auth;
pub mod scopes;
pub mod credentials;

use error::GOErr;
use auth::{Token, JwtClaims};
use credentials::Credentials;

use std::io::Read;
use smpl_jwt::Jwt;
use curl::easy::{Easy, List};

const DEFAULT_URL: &'static str = "https://www.googleapis.com/oauth2/v4/token";

fn form_body(body: String) -> String {
    format!("grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Ajwt-bearer&assertion={}", body)
}

/// Get Token which can be used to authenticate further request
/// ### Example
///
/// ```
/// extern crate smpl_jwt;
/// extern crate goauth;
/// #[macro_use]
/// extern crate log;
///
/// use goauth::auth::JwtClaims;
/// use goauth::scopes::Scope;
/// use goauth::get_token_legacy;
/// use smpl_jwt::{RSAKey, Jwt};
///
/// fn main() {
///   let token_url = "https://www.googleapis.com/oauth2/v4/token";
///   let iss = "some_iss"; // https://developers.google.com/identity/protocols/OAuth2ServiceAccount
///   let private_key_file = "random_rsa_for_testing";
///
///   let claims = JwtClaims::new(String::from(iss),
///                              Scope::DevStorageReadWrite,
///                              String::from(token_url),
///                              None, None);
///   let key = match RSAKey::from_pem(private_key_file) {
///     Ok(x) => x,
///     Err(e) => panic!("{}", e)
///   };
///   let jwt = Jwt::new(claims, key, None);
///   match get_token_legacy(&jwt, None) {
///     Ok(x) => debug!("{}", x),
///     Err(e) => debug!("{}", e)
///   };
/// }
///
/// ```
pub fn get_token_legacy(jwt: &Jwt<JwtClaims>, url: Option<&str>) -> Result<Token, GOErr> {
    let mut token: Result<Token, GOErr>;
    let request_body = form_body(jwt.finalize()?);
    let mut data = request_body.as_bytes();

    let mut easy = Easy::new();
    easy.url(url.unwrap_or(DEFAULT_URL))?;

    easy.post(true)?;
    easy.post_field_size(data.len() as u64)?;

    let mut list = List::new();
    list.append("Content-Type: application/x-www-form-urlencoded")?;

    {
        let mut transfer = easy.transfer();
        transfer.read_function(|buf| {
            Ok(data.read(buf).unwrap_or(0))
        })?;
        transfer.write_function(|data| {
            let response_data = std::str::from_utf8(data).expect("No response");
            token = Token::from_str(response_data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }
    token
}

pub fn get_token_as_string_legacy(jwt: &Jwt<JwtClaims>, url: Option<&str>) -> Result<String, GOErr> {
    Ok(serde_json::to_string(&get_token_legacy(jwt, url)?)?)
}

pub fn get_token_as_string(jwt: &Jwt<JwtClaims>, credentials: &Credentials) -> Result<String, GOErr> {
    Ok(serde_json::to_string(&get_token_with_creds(jwt, credentials)?)?)
}

#[test]
fn test_jwt_encode() {
    use auth::JwtClaims;
    use smpl_jwt::{RSAKey, Jwt};
    use scopes::Scope;

    let token_url = "https://www.googleapis.com/oauth2/v4/token";
    let iss = "some_iss"; // https://developers.google.com/identity/protocols/OAuth2ServiceAccount
    let private_key_file = "random_rsa_for_testing";


    let claims = JwtClaims::new(String::from(iss),
                                Scope::DevStorageReadWrite,
                                String::from(token_url),
                                Some(1482317385), Some(3600));
    let key = match RSAKey::from_pem(private_key_file) {
        Ok(x) => x,
        Err(e) => panic!("{}", e)
    };
    let jwt = Jwt::new(claims, key, None);
    assert_eq!(jwt.finalize().unwrap(), "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzb21lX2lzcyIsInNjb3BlIjoiaHR0cHM6Ly93d3cuZ29vZ2xlYXBpcy5jb20vYXV0aC9kZXZzdG9yYWdlLnJlYWRfd3JpdGUiLCJhdWQiOiJodHRwczovL3d3dy5nb29nbGVhcGlzLmNvbS9vYXV0aDIvdjQvdG9rZW4iLCJleHAiOjE0ODIzMjA5ODUsImlhdCI6MTQ4MjMxNzM4NX0.R9KMrV8j_g-JmCmjUauFtXxwe7Ho9ZeLim1Bm7FICn68kn0fNbyFE8Cd4kivx-WXqd25LZU6N-JFXSFGBLseor0zv_YTrTtEN4xCJ1jJscztBx_g5bRZ4FlCDEz05GI6aKZl2EGjd96Ofppi_ecLS__rnHGuq8zKUFznZtAST9dxuJ-9nlEKoW5VOGXadR9GnskHHRtwuX-ciP87blC7g8rLu14x-L3hThUDmz_uFky_8Po7xLpa_J-hmgRxGswZBBqutVnxUJZ3oMayb4hGR0r5JTQ7ovlAacU85kMoAos9tOoAE-R04oqY_AYuKQxRnEWALOYg0PXShdK302ieng");
}

#[test]
fn get_token_test() {
    //  This test will always pass, output is logged via debug macro
    use auth::JwtClaims;
    use smpl_jwt::{RSAKey, Jwt};
    use scopes::Scope;

    let token_url = "https://www.googleapis.com/oauth2/v4/token";
    let iss = "some_iss"; // https://developers.google.com/identity/protocols/OAuth2ServiceAccount
    let private_key_file = "random_rsa_for_testing";

    let claims = JwtClaims::new(String::from(iss),
                                Scope::DevStorageReadWrite,
                                String::from(token_url),
                                None, None);
    let key = match RSAKey::from_pem(private_key_file) {
        Ok(x) => x,
        Err(e) => panic!("{}", e)
    };
    let jwt = Jwt::new(claims, key, None);
    match get_token_legacy(&jwt, None) {
        Ok(x) => debug!("{}", x),
        Err(e) => debug!("{}", e)
    };
}


/// Get Token which can be used to authenticate further request
/// ### Example
///
/// ```
/// extern crate smpl_jwt;
/// extern crate goauth;
/// #[macro_use]
/// extern crate log;
///
/// use goauth::auth::JwtClaims;
/// use goauth::scopes::Scope;
/// use goauth::credentials::Credentials;
/// use goauth::get_token_with_creds;
/// use smpl_jwt::Jwt;
///
/// fn main() {
///
///   let credentials = Credentials::from_file("dummy_credentials_file_for_tests.json").unwrap();
///
///   let claims = JwtClaims::new(credentials.iss(),
///                              Scope::DevStorageReadWrite,
///                              credentials.token_uri(),
///                              None, None);
///
///   let jwt = Jwt::new(claims, credentials.rsa_key().unwrap(), None);
///   match get_token_with_creds(&jwt, &credentials) {
///     Ok(x) => debug!("{}", x),
///     Err(e) => debug!("{}", e)
///   };
/// }
///
/// ```
pub fn get_token_with_creds(jwt: &Jwt<JwtClaims>, credentials: &Credentials) -> Result<Token, GOErr> {
    let mut token: Result<Token, GOErr> = Err(GOErr::Unknown);
    let request_body = form_body(jwt.finalize()?);
    let mut data = request_body.as_bytes();

    let mut easy = Easy::new();
    easy.url(&credentials.token_uri())?;

    easy.post(true)?;
    easy.post_field_size(data.len() as u64)?;

    let mut list = List::new();
    list.append("Content-Type: application/x-www-form-urlencoded")?;

    {
        let mut transfer = easy.transfer();
        transfer.read_function(|buf| {
            Ok(data.read(buf).unwrap_or(0))
        })?;
        transfer.write_function(|data| {
            let response_data = std::str::from_utf8(data).expect("No response");
            token = Token::from_str(response_data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }
    token
}
