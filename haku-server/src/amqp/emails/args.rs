use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrder {
  pub order_number: String,
  pub order_created_at: DateTime<Utc>,
  pub customer_name: String,
}

impl CreateOrder {
  pub fn formatted_dt(&self) -> String {
    let taipei = FixedOffset::east(8 * 3600);

    self
      .order_created_at
      .with_timezone(&taipei)
      .format("%Y-%m-%d %H:%M:%S")
      .to_string()
  }
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

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderContent {
  // 顧客資料
  pub customer_name: String,

  // 訂單資料
  pub order_number: String,
  pub products: Vec<Product>,
  pub products_subtotal: i64,
  pub promotion_discount: i64, // 負數
  pub delivery_fee: i64,       // 實收運費（運費 - 運費減免）
  pub total: i64,
  pub order_created_at: DateTime<Utc>,

  // 收件資料
  pub addressee_name: String,
  pub package_numbers: String,
  pub logistics_type_name: String,
  pub branch_name: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Product {
  pub name: String,
  pub price: i64,
  pub amount: i64,
  pub subtotal: i64,
  pub picture_url: String,
}

impl OrderContent {
  pub fn formatted_dt(&self) -> String {
    let taipei = FixedOffset::east(8 * 3600);

    self
      .order_created_at
      .with_timezone(&taipei)
      .format("%Y-%m-%d %H:%M:%S")
      .to_string()
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequiredData {
  // 品牌資料
  pub brand_name: String,
  pub banner: String,
  pub homepage_link: String,
  pub facebook_link: String,
  pub instagram_link: String,
}
