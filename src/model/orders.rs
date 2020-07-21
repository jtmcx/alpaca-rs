use chrono::{DateTime, Utc};
use rug::{Assign, Float};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// todo ...
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    /// todo ...
    Buy,
    /// todo ...
    Sell,
}

/// todo ...
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    /// todo ...
    Market,
    /// todo ...
    Limit,
    /// todo ...
    Stop,
    /// todo ...
    StopLimit,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
    /// A day order is eligible for execution only on the day it is
    /// live. By default, the order is only valid during Regular Trading
    /// Hours (9:30am - 4:00pm ET). If unfilled after the closing auction,
    /// it is automatically canceled. If submitted after the close,
    /// it is queued and submitted the following trading day. However,
    /// if marked as eligible for extended hours, the order can also
    /// execute during supported extended hours.
    Day,
    /// The order is good until canceled. Non-marketable GTC limit
    /// orders are subject to price adjustments to offset corporate
    /// actions affecting the issue. We do not currently support Do Not
    /// Reduce(DNR) orders to opt out of such price adjustments.
    Gtc,
    /// Use this TIF with a market/limit order type to submit “market
    /// on open” (MOO) and “limit on open” (LOO) orders. This order
    /// is eligible to execute only in the market opening auction. Any
    /// unfilled orders after the open will be cancelled. OPG orders
    /// submitted after 9:28am but before 7:00pm ET will be rejected. OPG
    /// orders submitted after 7:00pm will be queued and routed to the
    /// following day’s opening auction. On open/on close orders are
    /// routed to the primary exchange. Such orders do not necessarily
    /// execute exactly at 9:30am / 4:00pm ET but execute per the
    /// exchange’s auction rules.
    Opg,
    /// Use this TIF with a market/limit order type to submit “market on
    /// close” (MOC) and “limit on close” (LOC) orders. This order
    /// is eligible to execute only in the market closing auction. Any
    /// unfilled orders after the close will be cancelled. CLS orders
    /// submitted after 3:50pm but before 7:00pm ET will be rejected. CLS
    /// orders submitted after 7:00pm will be queued and routed to the
    /// following day’s closing auction. Only available with API v2.
    Cls,
    /// An Immediate Or Cancel (IOC) order requires all or part of the
    /// order to be executed immediately. Any unfilled portion of the
    /// order is canceled. Only available with API v2.
    Ioc,
    /// A Fill or Kill (FOK) order is only executed if the entire order
    /// quantity can be filled, otherwise the order is canceled. Only
    /// available with API v2.
    Fok,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    /// The order has been received by Alpaca, and routed to exchanges
    /// for execution. This is the usual initial state of an order.
    New,
    /// The order has been partially filled.
    PartiallyFilled,
    /// The order has been filled, and no further updates will occur
    /// for the order.
    Filled,
    /// The order is done executing for the day, and will not receive
    /// further updates until the next trading day.
    DoneForDay,
    /// The order has been canceled, and no further updates will occur
    /// for the order. This can be either due to a cancel request by
    /// the user, or the order has been canceled by the exchanges due
    /// to its time-in-force.
    Canceled,
    /// The order has expired, and no further updates will occur for
    /// the order.
    Expired,
    /// The order was replaced by another order, or was updated due to
    /// a market event such as corporate action.
    Replaced,
    /// The order is waiting to be canceled.
    PendingCancel,
    /// The order is waiting to be replaced by another order. The order
    /// will reject cancel request while in this state.
    PendingReplace,
    /// The order has been received by Alpaca, but hasn’t yet been
    /// routed to the execution venue. This state only occurs on rare
    /// occasions.
    Accepted,
    /// The order has been received by Alpaca, and routed to the
    /// exchanges, but has not yet been accepted for execution. This
    /// state only occurs on rare occasions.
    PendingNew,
    /// The order has been received by exchanges, and is evaluated for
    /// pricing. This state only occurs on rare occasions.
    AcceptedForBidding,
    /// The order has been stopped, and a trade is guaranteed for the
    /// order, usually at a stated price or better, but has not yet
    /// occurred. This state only occurs on rare occasions.
    Stopped,
    /// The order has been rejected, and no further updates will occur
    /// for the order. This state occurs on rare occasions and may occur
    /// based on various conditions decided by the exchanges.
    Rejected,
    /// The order has been suspended, and is not eligible for
    /// trading. This state only occurs on rare occasions.
    Suspended,
    /// The order has been completed for the day (either filled or
    /// done for day), but remaining settlement calculations are still
    /// pending. This state only occurs on rare occasions.
    Calculated,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderClass {
    /// todo ...
    Simple,
    /// todo ...
    Bracket,
    /// todo ...
    Oco,
    /// todo ...
    Oto,
}

/// todo ...
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    /// Order id.
    pub id: Uuid,
    /// Client unique order id.
    pub client_order_id: String,
    /// todo ...
    pub created_at: DateTime<Utc>,
    /// todo ...
    pub updated_at: Option<DateTime<Utc>>,
    /// todo ...
    pub submitted_at: Option<DateTime<Utc>>,
    /// todo ...
    pub filled_at: Option<DateTime<Utc>>,
    /// todo ...
    pub expired_at: Option<DateTime<Utc>>,
    /// todo ...
    pub canceled_at: Option<DateTime<Utc>>,
    /// todo ...
    pub failed_at: Option<DateTime<Utc>>,
    /// Asset id.
    pub asset_id: Uuid,
    /// Asset symbol.
    pub symbol: String,
    /// Asset class.
    pub asset_class: String,
    /// todo ...
    #[serde(with = "super::serde::float")]
    pub qty: Float,
    /// todo ...
    #[serde(with = "super::serde::float")]
    pub filled_qty: Float,
    /// todo ...
    pub r#type: OrderType,
    /// todo ...
    pub side: Side,
    // TIME IN FORCE
    pub time_in_force: TimeInForce,
    /// todo ...
    #[serde(with = "super::serde::float_optional")]
    pub limit_price: Option<Float>,
    /// todo ...
    #[serde(with = "super::serde::float_optional")]
    pub stop_price: Option<Float>,
    /// todo ...
    #[serde(with = "super::serde::float_optional")]
    pub filled_avg_price: Option<Float>,
    /// todo ...
    pub status: OrderStatus,
    /// todo ...
    pub extended_hours: bool,
    /// todo ...
    pub legs: Option<Vec<Order>>,
}

/// todo ...
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderTakeProfit {
    /// todo ...
    #[serde(with = "super::serde::float")]
    pub limit_price: Float,
}

/// todo ...
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderStopLoss {
    /// todo ...
    #[serde(with = "super::serde::float")]
    pub stop_price: Float,
    /// todo ...
    #[serde(with = "super::serde::float_optional")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<Float>,
}

/// todo ...
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderRequest {
    /// Symbol or asset ID to identify the asset to trade.
    pub symbol: String,
    /// The number of shares to trade.
    #[serde(with = "super::serde::float")]
    pub qty: Float,
    /// todo ...
    pub side: Side,
    /// todo ...
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// todo ...
    pub time_in_force: TimeInForce,
    /// Limit price.  Required if `type` is `OrderType::Limit` or
    /// `OrderType::StopLimit`.
    #[serde(with = "super::serde::float_optional")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<Float>,
    /// Stop price.  Required if `type` is `OrderType::Stop` or
    /// `OrderType::StopLimit`.
    #[serde(with = "super::serde::float_optional")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<Float>,
    /// todo ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_hours: Option<bool>,
    /// todo ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// todo ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_class: Option<OrderClass>,
    /// todo ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<OrderTakeProfit>,
    /// todo ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<OrderStopLoss>,
}

impl OrderRequest {
    fn default() -> OrderRequest {
        OrderRequest {
            symbol: String::new(),
            qty: Float::with_val(53, 0),
            side: Side::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Day,
            limit_price: None,
            stop_price: None,
            extended_hours: None,
            client_order_id: None,
            order_class: None,
            take_profit: None,
            stop_loss: None,
        }
    }

    pub fn buy<T, U>(symbol: T, qty: U) -> OrderRequest
    where
        T: Into<String>,
        Float: rug::Assign<U>,
    {
        OrderRequest::default()
            .side(Side::Buy)
            .symbol(symbol)
            .qty(qty)
    }

    pub fn sell<T, U>(symbol: T, qty: U) -> OrderRequest
    where
        T: Into<String>,
        Float: rug::Assign<U>,
    {
        OrderRequest::default()
            .side(Side::Sell)
            .symbol(symbol)
            .qty(qty)
    }

    pub fn symbol<T>(mut self, symbol: T) -> Self
    where
        T: Into<String>,
    {
        self.symbol = symbol.into();
        self
    }

    pub fn qty<T>(mut self, qty: T) -> Self
    where
        Float: rug::Assign<T>,
    {
        self.qty.assign(qty);
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = order_type;
        self
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = time_in_force;
        self
    }

    pub fn limit_price<T>(mut self, limit_price: T) -> Self
    where
        Float: rug::Assign<T>,
    {
        self.limit_price = Some(Float::with_val(53, limit_price));
        self
    }

    pub fn stop_price<T>(mut self, stop_price: T) -> Self
    where
        Float: rug::Assign<T>,
    {
        self.stop_price = Some(Float::with_val(53, stop_price));
        self
    }

    pub fn extended_hours(mut self, extended_hours: bool) -> Self {
        self.extended_hours = Some(extended_hours);
        self
    }

    pub fn client_order_id(mut self, client_order_id: String) -> Self {
        self.client_order_id = Some(client_order_id);
        self
    }

    pub fn order_class(mut self, order_class: OrderClass) -> Self {
        self.order_class = Some(order_class);
        self
    }

    pub fn take_profit(mut self, limit_price: Float) -> Self {
        self.take_profit = Some(OrderTakeProfit { limit_price });
        self
    }

    pub fn stop_loss(mut self, stop_price: Float, limit_price: Option<Float>) -> Self {
        self.stop_loss = Some(OrderStopLoss {
            stop_price,
            limit_price,
        });
        self
    }
}

/// todo ...
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderReplace {
    /// todo ...
    #[serde(with = "super::serde::float_optional")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<Float>,
    /// todo ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// Limit price.  Required if `type` is `OrderType::Limit` or
    /// `OrderType::StopLimit`.
    #[serde(with = "super::serde::float_optional")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<Float>,
    /// Stop price.  Required if `type` is `OrderType::Stop` or
    /// `OrderType::StopLimit`.
    #[serde(with = "super::serde::float_optional")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<Float>,
    /// todo ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

impl OrderReplace {
    pub fn new() -> OrderReplace {
        OrderReplace {
            qty: None,
            time_in_force: None,
            limit_price: None,
            stop_price: None,
            client_order_id: None
        }
    }

    pub fn qty<T>(mut self, qty: T) -> Self
    where
        Float: rug::Assign<T>,
    {
        self.qty = Some(Float::with_val(53, qty));
        self
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    pub fn limit_price<T>(mut self, limit_price: T) -> Self
    where
        Float: rug::Assign<T>,
    {
        self.limit_price = Some(Float::with_val(53, limit_price));
        self
    }

    pub fn stop_price<T>(mut self, stop_price: T) -> Self
    where
        Float: rug::Assign<T>,
    {
        self.stop_price = Some(Float::with_val(53, stop_price));
        self
    }

    pub fn client_order_id<T>(mut self, client_order_id: T) -> Self
    where
        T: Into<String>
    {
        self.client_order_id = Some(client_order_id.into());
        self
    }
}

