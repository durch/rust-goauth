//! Defines a `TokenFetcher` struct that will automatically refresh tokens
//! at some configured time prior to the token's expiration.

use crate::auth::{JwtClaims, Token};
use crate::credentials::Credentials;
use crate::{get_token_with_client, Result};

use reqwest::Client;
use smpl_jwt::Jwt;
use time::{Duration, OffsetDateTime};

/// A `TokenFetcher` stores a `Token` on first fetch and will continue returning
/// that token until it needs to be refreshed, as determined by the token's
/// `expires_in` field and the configured `refresh_buffer`.
///
/// Specifically on each token fetch request, it will check the current time
/// against the expected time the currently stored token will expire. If it
/// is within the `refresh_buffer` window, it will fetch a new token, store
/// that (along with the new expired time), and return the new token.
pub struct TokenFetcher {
    client: Client,
    jwt: Jwt<JwtClaims>,
    credentials: Credentials,
    token_state: Option<TokenState>,
    refresh_buffer: Duration,
}

struct TokenState {
    /// The currently stored token
    token: Token,
    /// The lower bound of the time at which the token needs to be refreshed
    refresh_at: OffsetDateTime,
}

impl TokenFetcher {
    pub fn new(
        jwt: Jwt<JwtClaims>,
        credentials: Credentials,
        refresh_buffer: Duration
    ) -> TokenFetcher {
        TokenFetcher::with_client(Client::new(), jwt, credentials, refresh_buffer)
    }

    pub fn with_client(
        client: Client,
        jwt: Jwt<JwtClaims>,
        credentials: Credentials,
        refresh_buffer: Duration
    ) -> TokenFetcher {
        let token_state = None;

        TokenFetcher {
            client,
            jwt,
            credentials,
            token_state,
            refresh_buffer,
        }
    }

    /// Returns a token if the token is still considered "valid" per the
    /// currently stored token's `expires_in` field and the configured
    /// `refresh_buffer`. If it is, return the stored token. If not,
    /// fetch a new token, store it, and return the new token.
    pub async fn fetch_token(&mut self) -> Result<Token> {
        match &self.token_state {
            // First time calling `fetch_token` since initialization, so fetch
            // a token.
            None => self.get_token().await,
            Some(token_state) => {
                let now = OffsetDateTime::now_utc();

                if now >= token_state.refresh_at {
                    // We have an existing token but it is time to refresh it
                    self.get_token().await
                } else {
                    // We have an existing, valid token, so return immediately
                    Ok(token_state.token.clone())
                }
            },
        }
    }

    /// Refresh the token
    async fn get_token(&mut self) -> Result<Token> {
        let now = OffsetDateTime::now_utc();

        let token = get_token_with_client(&self.client, &self.jwt, &self.credentials).await?;
        let expires_in = Duration::new(token.expires_in().into(), 0);

        assert!(expires_in >= self.refresh_buffer, "Received a token whose expires_in is less than the configured refresh buffer!");

        let refresh_at = now + (expires_in - self.refresh_buffer);
        let token_state = TokenState {
            token: token.clone(),
            refresh_at,
        };

        self.token_state.replace(token_state);
        Ok(token)
    }
}
