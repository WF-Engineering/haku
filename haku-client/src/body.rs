use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as Jsonb;

use crate::args::RequiredData;

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

impl EmailNotification {
  pub fn builder() -> EmailNotificationBuilder {
    EmailNotificationBuilder(Self::default())
  }
}

impl Default for EmailNotification {
  fn default() -> Self {
    Self {
      brand_id: i64::default(),
      template: Template::default(),
      customer: Customer::default(),
      sender: Sender::default(),
      required_args: Jsonb::default(),
      other_args: Jsonb::default(),
      created_at: Utc.ymd(2020, 1, 1).and_hms(0, 0, 0),
    }
  }
}

#[derive(Debug)]
pub struct EmailNotificationBuilder(EmailNotification);

impl EmailNotificationBuilder {
  pub fn brand_id(self, brand_id: i64) -> Self {
    let mut inner = self.0;
    inner.brand_id = brand_id;
    Self(inner)
  }

  pub fn template_id(self, template_id: i32) -> Self {
    let mut inner = self.0;
    inner.template.id = template_id;
    Self(inner)
  }

  pub fn template_mode(self, template_mode: TemplateMode) -> Self {
    let mut inner = self.0;
    inner.template.mode = template_mode;
    Self(inner)
  }

  pub fn template_subject(self, template_subject: String) -> Self {
    let mut inner = self.0;
    inner.template.subject = template_subject;
    Self(inner)
  }

  pub fn customer_email(self, customer_email: String) -> Self {
    let mut inner = self.0;
    inner.customer.email = customer_email;
    Self(inner)
  }

  pub fn customer_name(self, customer_name: String) -> Self {
    let mut inner = self.0;
    inner.customer.name = customer_name;
    Self(inner)
  }

  pub fn sender_id(self, sender_id: i32) -> Self {
    let mut inner = self.0;
    inner.sender.id = sender_id;
    Self(inner)
  }

  pub fn sender_name(self, sender_name: String) -> Self {
    let mut inner = self.0;
    inner.sender.name = sender_name;
    Self(inner)
  }

  pub fn sender_email(self, sender_email: String) -> Self {
    let mut inner = self.0;
    inner.sender.email = sender_email;
    Self(inner)
  }

  pub fn required_args(self, args: Jsonb) -> Self {
    let mut inner = self.0;
    inner.required_args = args;
    Self(inner)
  }

  // TODO: 完全置換成新通知系統前，先塞 default 值
  pub fn default_required_args(self) -> Self {
    let mut inner = self.0;
    inner.required_args = serde_json::to_value(RequiredData::default()).unwrap();
    Self(inner)
  }

  pub fn other_args(self, args: Jsonb) -> Self {
    let mut inner = self.0;
    inner.other_args = args;
    Self(inner)
  }

  pub fn created_at(self, created_at: DateTime<Utc>) -> Self {
    let mut inner = self.0;
    inner.created_at = created_at;
    Self(inner)
  }

  pub fn create(self) -> EmailNotification {
    self.0
  }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Template {
  pub id: i32,
  pub mode: TemplateMode,
  pub subject: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Customer {
  pub email: String,
  pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
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

impl Default for TemplateMode {
  fn default() -> Self {
    Self::CreateOrder
  }
}
