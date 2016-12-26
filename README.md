[![Build Status](https://travis-ci.org/durch/rust-goauth.svg?branch=master)](https://travis-ci.org/durch/rust-goauth)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/durch/rust-goauth/blob/master/LICENSE)
[![](http://meritbadge.herokuapp.com/goauth)](https://crates.io/crates/goauth)
[![Join the chat at https://gitter.im/durch/rust-goauth](https://badges.gitter.im/durch/rust-goauth.svg)](https://gitter.im/durch/rust-goauth?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

## rust-goauth [[docs](https://durch.github.io/rust-goauth)]

Crate for using [OAuth 2.0 with Server to Server Applications](https://developers.google.com/identity/protocols/OAuth2ServiceAccount) for Google Cloud Engine, with tentative support for all supported [Scopes](https://durch.github.io/rust-goauth/goauth/scopes/enum.Scope.html).

Provides a serialisable [Token](https://durch.github.io/rust-goauth/goauth/auth/struct.Token.html) struct for use in other applications that require authenticated interactions with Google Cloud.

### Usage

```

extern crate smpl_jwt;
extern crate goauth;
#[macro_use]
extern crate log;

use goauth::auth::JwtClaims;
use goauth::scopes::Scope;
use goauth::get_token;
use smpl_jwt::{RSAKey, Jwt};

fn main() {
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
  match get_token(&jwt, None) {
    Ok(x) => debug!("{}", x),
    Err(e) => debug!("{}", e)
  };
}```
