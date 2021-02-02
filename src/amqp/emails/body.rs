use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as Jsonb;

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailNotification {
  pub brand_id: i64,
  pub template: Template,
  pub customer: Customer,
  pub sender: Sender,
  pub required_args: Jsonb,
  pub other_args: Jsonb,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
  pub id: i32,
  pub mode: TemplateMode,
  pub subject: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
  pub email: String,
  pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sender {
  pub id: i32,
  pub name: String,
  pub email: String,
}

#[derive(Debug, Display, EnumString, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TemplateMode {
  /// 訂單成立成功
  CreateOrder,
  /// 倉儲已出貨
  WarehouseDelivered,
  /// 物流狀態改變
  LogisticsStatusChanged,
  /// 訂單已退款
  PaymentReturned,
  /// 發送註冊驗證信
  AccountRegister,
  /// 發送忘記密碼
  AccountForgotPassword,
  /// 發送提早出貨通知
  EarlyDelivery,
  /// 物流已出貨通知
  ShippingInfo,
  /// 通知物流已到指定商店
  CvsArriveInfo,
  /// 訂單評價信
  CsatSurvey,
  /// 訂單成立成功
  NewCreateOrder,
}
