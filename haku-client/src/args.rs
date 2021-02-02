use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RequiredData {
  // 品牌資料
  pub brand_name: String,
  pub banner: String,
  pub homepage_link: String,
  pub facebook_link: String,
  pub instagram_link: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrder {
  pub order_number: String,
  pub order_created_at: DateTime<Utc>,
  pub customer_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Signup {
  pub brand_name: String,
  pub customer_name: String,
  pub link: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotPassword {
  pub customer_name: String,
  pub link: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EarlyDelivery {
  pub customer_name: String,
  pub order_number: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShippingInfo {
  pub customer_name: String,
  pub order_number: String,
  pub package_numbers: String,
  pub logistics_type_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CvsArriveInfo {
  pub customer_name: String,
  pub order_number: String,
  pub branch_name: String,
  pub addressee_name: String,
  pub package_number: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CsatSurvey {
  pub customer_name: String,
  pub order_number: String,
  pub link: String,
}
