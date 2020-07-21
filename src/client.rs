use super::model::*;

use std::convert::TryInto;
use url::Url;
use uuid::Uuid;
//use simple_error::SimpleError;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};


// type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub struct Client {
    /// todo: docs ...
    client: reqwest::Client,
    /// todo: docs ...
    endpoint: Url,
}

/// todo: docs ...
pub struct ClientBuilder {
    /// todo: docs ...
    endpoint: Url,
    /// todo: docs ...
    key_id: Option<String>,
    /// todo: docs ...
    secret_key: Option<String>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl ClientBuilder {

    /// todo: docs ...
    pub fn new() -> Self {
        ClientBuilder {
            endpoint: Client::paper_endpoint(),
            key_id: None,
            secret_key: None,
        }
    }

    /// todo: docs ...
    pub fn build(self) -> Result<Client> {
        // Extract the secrets.
        let key_id = self.key_id.ok_or("Missing alpaca key id")?;
        let secret_key = self.secret_key.ok_or("Missing alpaca secret key")?;

        // Setup the default headers with the secrets.
        let mut headers = HeaderMap::new();
        headers.insert("APCA-API-KEY-ID",
                       HeaderValue::from_str(&key_id)?);
        headers.insert("APCA-API-SECRET-KEY",
                       HeaderValue::from_str(&secret_key)?);

        // Build the HTTP client.
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Client {
            client: client,
            endpoint: self.endpoint,
        })
    }

    /// todo: docs ...
    pub fn endpoint<U>(mut self, endpoint: U)
            -> std::result::Result<Self, <U as TryInto<Url>>::Error>
        where U: TryInto<Url>
    {
        self.endpoint = endpoint.try_into()?;
        Ok(self)
    }

    /// todo: docs ...
    pub fn key_id<T>(mut self, key_id: T) -> Self
        where T: Into<String>
    {
        self.key_id = Some(key_id.into());
        self
    }

    /// todo: docs ...
    pub fn secret_key<T>(mut self, secret_key: T) -> Self
        where T: Into<String>
    {
        self.secret_key = Some(secret_key.into());
        self
    }
}

/*
struct Builder {
    url: Url
}

impl Builder {

    /// todo: docs ...
    fn path(mut self, input: &str) -> Result<Self> {
        self.url = self.url.join(input)?;
        Ok(self)
    }
}
*/

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// todo: docs ...
    pub fn paper_endpoint() -> Url {
        let endpoint = "https://paper-api.alpaca.markets";
        Url::parse(endpoint).unwrap()
    }

    async fn handle_request<T>(request: reqwest::RequestBuilder) -> Result<T>
        where for<'de> T: serde::Deserialize<'de>
    {
        let response = request.send().await?;

        // This should be clarified a little more. Which status
        // codes return a json object, and which do not? Right now,
        // assuming that anything non-2xx returns a json object with
        // error information may mask internal server errors and other
        // connection problems.

        match response.status().is_success() {
            true  => Ok(response.json::<T>().await?),
            false => Err(Box::new(response.json::<Error>().await?)),
        }
    }

    pub async fn get_account(&self) -> Result<Account> {
        let path = self.endpoint.join("/v2/account")?;
        Self::handle_request(self.client.get(path)).await
    }

    pub async fn get_orders(&self) -> Result<Vec<Order>> {
        let path = self.endpoint.join("/v2/orders")?;
        Self::handle_request(self.client.get(path)).await
    }

    pub async fn get_order(&self, id: &Uuid) -> Result<Order> {
        let path = self.endpoint
            .join("/v2/orders/")?
            .join(&id.to_hyphenated_ref().to_string())?;

        Self::handle_request(self.client.get(path)).await
    }

    pub async fn request_order(&self, req: &OrderRequest) -> Result<Order> {
        let request = self.client
            .post(self.endpoint.join("/v2/orders")?)
            .json(req);

        Self::handle_request(request).await
    }

    pub async fn replace_order(&self, id: Uuid, args: &OrderReplace) -> Result<Order> {
        let path = self.endpoint
            .join("/v2/orders/")?
            .join(&id.to_hyphenated_ref().to_string())?;

        let request = self.client
            .patch(path)
            .json(args);

        Self::handle_request(request).await
    }

    pub async fn cancel_order(&self, id: Uuid) -> Result<()> {
        let path = self.endpoint
            .join("/v2/orders/")?
            .join(&id.to_hyphenated_ref().to_string())?;

        Self::handle_request(self.client.delete(path)).await
    }

    /// todo: mult-response??
    pub async fn cancel_all_orders(&self) -> Result<()> {
        let request = self.client
            .delete(self.endpoint.join("/v2/orders")?);
        request.send().await?;
        Ok(())
    }
}
