use actix_web::{HttpResponse, ResponseError};

pub type HakuResult<T> = Result<T, HakuError>;

#[derive(Debug, thiserror::Error)]
pub enum HakuError {
  #[error("lapin error: {0:?}")]
  FromLapin(lapin::Error),
}

impl ResponseError for HakuError {
  fn error_response(&self) -> HttpResponse {
    error!("Haku error: {:?}", self);
    HttpResponse::InternalServerError().finish()
  }
}

impl From<lapin::Error> for HakuError {
  fn from(err: lapin::Error) -> Self {
    Self::FromLapin(err)
  }
}
