#[macro_use]
extern crate log;

#[macro_use]
extern crate strum_macros;

pub mod args;
pub mod body;
pub mod errors;

use bytes::Bytes;
use errors::{HakuError, HakuResult};
use reqwest::{header::HeaderMap, Response};
use serde::{de::DeserializeOwned, Serialize};

pub mod header {
  pub use reqwest::header::*;
}

#[derive(Clone)]
pub struct HakuClient {
  pub server_url: String,
  headers: HeaderMap,
}

impl HakuClient {
  pub fn new(server_url: String) -> Self {
    Self {
      server_url,
      headers: Self::default_headers(),
    }
  }

  fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCEPT, "application/json".parse().unwrap());
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    headers
  }

  fn make_url<Path: Into<String>>(&self, path: Path) -> String {
    let url = format!("{}{}", self.server_url, path.into());
    debug!("make url result: {}", url);
    url
  }

  async fn check_resp(&self, resp: Response) -> Result<Bytes, HakuError> {
    debug!("resp: {:?}", resp.status());

    match resp.status().is_success() {
      true => {
        let bytes = resp.bytes().await;
        trace!("bytes: {:?}", &bytes);
        bytes.map_err(HakuError::from)
      }
      false => {
        warn!("status code: {}", resp.status());
        warn!("body: {:?}", resp.bytes().await);
        Ok(Bytes::new())
      }
    }
  }

  pub async fn post_bytes<Path: Into<String>, Body>(
    &self,
    path: Path,
    body: Body,
  ) -> Result<Bytes, HakuError>
  where
    Body: Serialize,
  {
    let client = reqwest::Client::new()
      .post(&self.make_url(path))
      .headers(self.headers.clone());

    debug!("ready to post bytes: {:?}", client);

    let resp = client.json(&body).send().await?;
    debug!("resp: {:?}", resp);

    self.check_resp(resp).await
  }

  pub async fn post<Path: Into<String>, T, Body>(
    &self,
    path: Path,
    body: Body,
  ) -> Result<T, HakuError>
  where
    T: DeserializeOwned,
    Body: Serialize,
  {
    let bytes = self.post_bytes(path, body).await?;

    serde_json::from_slice::<T>(&bytes).map_err(HakuError::from)
  }
}

impl HakuClient {
  pub async fn send_email(
    &self,
    body: body::EmailNotification,
  ) -> HakuResult<()> {
    debug!("prepare send email");

    debug!("body: {}", serde_json::to_string_pretty(&body).unwrap());
    let path = "/orders/create".to_string();
    self.post_bytes(path, body).await.map_err(HakuError::from)?;

    Ok(())
  }
}
