use chrono::{DateTime, Utc};
use rug::Float;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// The following are the possible account status values. Most likely,
/// the account status is `Active` unless there is any problem. The account
/// status may get in `AccountUpdated` when personal information is being
/// updated from the dashboard, in which case you may not be allowed
/// trading for a short period of time until the change is approved.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
    /// The account is onboarding.
    OnBoarding,
    /// The account application submission failed for some reason.
    SubmissionFailed,
    /// The account application has been submitted for review.
    Submitted,
    /// The account information is being updated.
    AccountUpdated,
    /// The final account approval is pending.
    ApprovalPending,
    /// The account is active for trading.
    Active,
    /// The account application has been rejected.
    Rejected
}

/// The account API serves important information related to an
/// account, including account status, funds available for trade,
/// funds available for withdrawal, and various flags relevant to an
/// account’s ability to trade. An account maybe be blocked for just
/// for trades (trades_blocked flag) or for both trades and transfers
/// (account_blocked flag) if Alpaca identifies the account to engaging in
/// any suspicious activity. Also, in accordance with FINRA’s pattern
/// day trading rule, an account may be flagged for pattern day trading
/// (pattern_day_trader flag), which would inhibit an account from
/// placing any further day-trades.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    /// Account ID.
    pub id: Uuid,
    /// Account number.
    pub account_number: String,
    /// Account status.
    pub status: AccountStatus,
    /// Currency code. For example "USD".
    pub currency: String,
    /// Cash balance.
    #[serde(with = "super::serde::float")]
    pub cash: Float,
    /// Whether or not the account has been flagged as a pattern day trader.
    pub pattern_day_trader: bool,
    /// User setting. If true, the account is not allowed to place orders.
    pub trade_suspended_by_user: bool,
    /// If true, the account is not allowed to place orders.
    pub trading_blocked: bool,
    /// If true, the account is not allowed to request money transfers.
    pub transfers_blocked: bool,
    /// If true, the account activity by user is prohibited.
    pub account_blocked: bool,
    /// Timestamp this account was created at.
    pub created_at: DateTime<Utc>,
    /// Flag to denote whether or not the account is permitted to short.
    pub shorting_enabled: bool,
    /// Real-time MtM value of all long positions held in the account.
    #[serde(with = "super::serde::float")]
    pub long_market_value: Float,
    /// Real-time MtM value of all short positions held in the account.
    #[serde(with = "super::serde::float")]
    pub short_market_value: Float,
    /// Cash + long_market_value + short_market_value.
    #[serde(with = "super::serde::float")]
    pub equity: Float,
    /// Equity as of previous trading day at 16:00:00 ET.
    #[serde(with = "super::serde::float")]
    pub last_equity: Float,
    /// Buying power multiplier that represents account margin
    /// classification; valid values 1 (standard limited margin account
    /// with 1x buying power), 2 (reg T margin account with 2x intraday
    /// and overnight buying power; this is the default for all non-PDT
    /// accounts with $2,000 or more equity), 4 (PDT account with 4x
    /// intraday buying power and 2x reg T overnight buying power)
    #[serde(with = "super::serde::float")]
    pub multiplier: Float,
    /// Current available $ buying power; If multiplier = 4, this is
    /// your daytrade buying power which is calculated as (last_equity -
    /// (last) maintenance_margin) * 4; If multiplier = 2, buying_power
    /// = max(equity – initial_margin,0) * 2; If multiplier = 1,
    /// buying_power = cash.
    #[serde(with = "super::serde::float")]
    pub buying_power: Float,
    /// Reg T initial margin requirement (continuously updated value).
    #[serde(with = "super::serde::float")]
    pub initial_margin: Float,
    /// Maintenance margin requirement (continuously updated value).
    #[serde(with = "super::serde::float")]
    pub maintenance_margin: Float,
    /// Value of special memorandum account (will be used at a later
    /// date to provide additional buying_power).
    #[serde(with = "super::serde::float")]
    pub sma: Float,
    /// The current number of daytrades that have been made in the last
    /// 5 trading days (inclusive of today).
    pub daytrade_count: i64,
    /// Your maintenance margin requirement on the previous trading day.
    #[serde(with = "super::serde::float")]
    pub last_maintenance_margin: Float,
    /// Your buying power for day trades (continuously updated value).
    #[serde(with = "super::serde::float")]
    pub daytrading_buying_power: Float,
    /// Your buying power under Regulation T (your excess equity -
    /// equity minus margin value - times your margin multiplier).
    #[serde(with = "super::serde::float")]
    pub regt_buying_power: Float,
}
