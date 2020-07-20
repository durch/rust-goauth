![build](https://github.com/durch/rust-goauth/workflows/Rust/badge.svg)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/durch/rust-goauth/blob/master/LICENSE)
[![](http://meritbadge.herokuapp.com/goauth)](https://crates.io/crates/goauth)

## rust-goauth [[docs](https://docs.rs/goauth)]

Crate for using [OAuth 2.0 with Server to Server Applications](https://developers.google.com/identity/protocols/OAuth2ServiceAccount) for Google Cloud Engine, with tentative support for all supported [Scopes](https://durch.github.io/rust-goauth/goauth/scopes/enum.Scope.html). Supports sync or async requests via Futures.

Provides a serialisable [Token](https://durch.github.io/rust-goauth/goauth/auth/struct.Token.html) struct for use in other applications that require authenticated interactions with Google Cloud.

### Usage

```rust,no_run
#[macro_use]
extern crate log;

use goauth::auth::JwtClaims;
use goauth::scopes::Scope;
use goauth::{get_token, get_token_blocking, GoErr};
use goauth::credentials::Credentials;
use smpl_jwt::{RSAKey, Jwt};

fn main() -> Result<(), GoErr>{
  let token_url = "https://www.googleapis.com/oauth2/v4/token";
  let iss = "<some-iss>"; // https://developers.google.com/identity/protocols/OAuth2ServiceAccount

  let credentials = Credentials::from_file("dummy_credentials_file_for_tests.json").unwrap();
  let claims = JwtClaims::new(String::from(iss),
                             &Scope::DevStorageReadWrite,
                             String::from(token_url),
                             None, None);
  let jwt = Jwt::new(claims, credentials.rsa_key().unwrap(), None);

  // Use async
  let token = async {
    match get_token(&jwt, &credentials).await {
      Ok(token) => token,
      Err(e) => panic!(e)
    }
  };

  // Or sync
  let token = get_token_blocking(&jwt, &credentials)?;

  Ok(())
  
}
```
