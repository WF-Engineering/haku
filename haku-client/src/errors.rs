use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;

pub type HakuResult<T> = Result<T, HakuError>;

#[derive(Debug)]
pub enum HakuError {
  DecodingError(JsonError),
  FromReqwest(ReqwestError),
}

impl From<ReqwestError> for HakuError {
  fn from(err: ReqwestError) -> Self {
    HakuError::FromReqwest(err)
  }
}

impl From<JsonError> for HakuError {
  fn from(err: JsonError) -> Self {
    HakuError::DecodingError(err)
  }
}
