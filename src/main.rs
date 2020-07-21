mod client;
mod model;

use model::*;

use serde_json;
use client::Client;
use std::error::Error;
use uuid::Uuid;
use rug::Float;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .endpoint(Client::paper_endpoint())?
        .key_id("")
        .secret_key("")
        .build()?;

    let orders = client.get_orders().await?;
    println!("{:?}", orders);
    println!("------");

    let request = OrderRequest::buy("IBM", 2)
        .order_type(OrderType::Limit)
        .limit_price(1);

    let order = client.request_order(&request).await?;
    println!("{:?}", order);
    println!("------");

    client.request_order(&request).await?;

    println!("{:?}", client.cancel_all_orders().await?);

    Ok(())
}
