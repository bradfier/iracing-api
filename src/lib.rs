use crate::auth::{AuthHash, AuthRequest};
use crate::types::SeriesList;
use reqwest::{ClientBuilder, Method, StatusCode};
use serde::{de::DeserializeOwned, Deserialize};
use thiserror::Error;
use types::{Cars, Member, Tracks};
use url::Url;
use crate::types::assets::{CarAssetsMap, TrackAssetsMap};

pub mod auth;
pub mod types;

const DEFAULT_BASE_URL: &str = "https://members-ng.iracing.com/";

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed initial authentication with iRacing: {}", .0.message)]
    UnsuccessfulAuth(UnsuccessfulAuth),
    #[error("Error making request: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Unknown error")]
    Unknown(String),
}

impl Error {
    pub fn is_unauthorized(&self) -> bool {
        match self {
            Error::RequestError(e) => e.status().is_some_and(|s| s == StatusCode::UNAUTHORIZED),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct IRRedirect {
    link: Url,
}

#[derive(Debug)]
pub struct Client {
    inner: reqwest::Client,
    base_url: Url,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SuccessfulAuth {
    pub authcode: String,
    #[serde(rename = "custId")]
    pub cust_id: i32,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnsuccessfulAuth {
    pub authcode: i32,
    pub inactive: bool,
    pub message: String,
    #[serde(rename = "verificationRequired")]
    pub verification_required: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AuthResponse {
    Success(SuccessfulAuth),
    Failure(UnsuccessfulAuth),
}

impl Client {
    /// Construct a new API client and authenticate with the iRacing API
    ///
    /// The iRacing developers have requested that API users re-authenticate as infrequently as
    /// possible, so try and re-use the constructed [`Client`] where possible.
    pub async fn login(email: &str, password: &str) -> Result<Self, Error> {
        let auth_body = auth::AuthRequest::new(email, password);
        Self::inner_login(auth_body).await
    }

    pub async fn login_with_hash(email: &str, auth_hash: AuthHash) -> Result<Self, Error> {
        let auth_body = auth::AuthRequest::new_from_hash(email, auth_hash);
        Self::inner_login(auth_body).await
    }

    async fn inner_login(request: AuthRequest) -> Result<Self, Error> {
        let client = ClientBuilder::new().cookie_store(true).build()?;
        let base_url: Url = DEFAULT_BASE_URL.parse().unwrap();

        let res: AuthResponse = client
            .request(Method::POST, base_url.join("auth").unwrap())
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        match res {
            AuthResponse::Success(_) => {
                // Successful auth should have set cookies on our client
                Ok(Self {
                    inner: client,
                    base_url,
                })
            }
            AuthResponse::Failure(unsuccessful) => {
                Err(Error::UnsuccessfulAuth(unsuccessful))
            }
        }

    }

    // iRacing returns API responses by serving them from S3 buckets, where you first receive a
    // redirect structure containing a signed URL which serves the content
    async fn fetch<T: DeserializeOwned>(&self, route: &str) -> Result<T, Error> {
        let redirect: IRRedirect = self
            .inner
            .get(self.base_url.join(route).unwrap())
            .send()
            .await?
            .json()
            .await?;
        Ok(self
            .inner
            .get(redirect.link)
            .send()
            .await?
            .json::<T>()
            .await?)
    }

    pub async fn get_member_info(&self) -> Result<Member, Error> {
        self.fetch("data/member/info").await
    }

    pub async fn get_car_info(&self) -> Result<Cars, Error> {
        Ok(Cars {
            inner: self.fetch("data/car/get").await?,
        })
    }

    pub async fn get_car_assets(&self) -> Result<CarAssetsMap, Error> {
        Ok(self.fetch("data/car/assets").await?)
    }

    pub async fn get_track_info(&self) -> Result<Tracks, Error> {
        Ok(Tracks {
            inner: self.fetch("data/track/get").await?,
        })
    }

    pub async fn get_track_assets(&self) -> Result<TrackAssetsMap, Error> {
        Ok(self.fetch("data/track/assets").await?)
    }

    pub async fn get_series_info(&self) -> Result<SeriesList, Error> {
        Ok(SeriesList {
            inner: self.fetch("data/series/seasons").await?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn get_test_client() -> Client {
        let username = std::env::var("TEST_USERNAME").unwrap();
        let auth_hash = std::env::var("AUTH_HASH").unwrap();
        Client::login_with_hash(&username, auth_hash).await.unwrap()
    }

    #[tokio::test]
    async fn client_login() {
        let c = get_test_client().await;
        let member = c.get_member_info().await.unwrap();
        // iRacing anonymises the email address even if you just authenticated with it
        assert_eq!(member.email, "b********2@g***l.c**");
    }

    #[tokio::test]
    async fn get_cars() {
        let c = get_test_client().await;
        let cars = c.get_car_info().await.unwrap();

        assert!(cars.inner.len() > 100);
    }

    #[tokio::test]
    async fn get_tracks() {
        let c = get_test_client().await;
        let tracks = c.get_track_info().await.unwrap();

        assert!(tracks.inner.len() > 300);
    }

    #[tokio::test]
    async fn get_series_info() {
        let c = get_test_client().await;
        let series = c.get_series_info().await.unwrap();

        assert!(series.inner.len() > 50);
    }
}
