use serde_json;
use std::io::prelude::*;
use std::fs::File;
use error::GOErr;
use smpl_jwt::RSAKey;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    #[serde(rename = "type")]
    t: String,
    project_id: String,
    private_key_id: String,
    private_key: String,
    client_email: String,
    client_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_x509_cert_url: String
}

impl Credentials {
    pub fn from_file(fp: &str) -> Result<Self, GOErr> {
        let mut f = File::open(fp)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        Ok(serde_json::from_slice(&buffer.as_slice())?)
    }

    pub fn from_str(s: &str) -> Result<Self, GOErr> {
        Ok(serde_json::from_str(s)?)
    }

    pub fn rsa_key(&self) -> Result<RSAKey, GOErr> {
        Ok(RSAKey::from_str(&self.private_key)?)
    }

    pub fn iss<'a>(&self) -> String {
        self.client_email.clone()
    }

    pub fn project<'a>(&self) -> String {
        self.project_id.clone()
    }

    pub fn token_uri<'a>(&self) -> String {
        self.token_uri.clone()
    }
}