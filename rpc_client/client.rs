use std::fmt;

use jsonrpc;
use serde_json;

use error::Error;

/// JSON RPC Client Wrapper
pub struct Client {
    client: jsonrpc::client::Client,
}

impl Client {
    fn new(url: String, user: String, pass: String) -> Client {
        Client {
            client: jsonrpc::client::Client::new(url, Some(user), Some(pass)),
        }
    }

    pub fn send_request(
        &self,
        name: String,
        params: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, Error> {
        let req = self
            .client
            .build_request(name, params);

        let res = self
            .client
            .send_request(&req)
            .and_then(|res| res.result())?;

        // TODO: map Vec<serde_json::Value> to custom structs
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let url = String::from("localhost:18443");
        let user = String::from("user");
        let pass = String::from("pass");

        let client = Client::new(url, user, pass);

        match client {
            Client { client: _ } => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_send_request() {
        let url = String::from("http://localhost:18443");
        let user = String::from("user");
        let pass = String::from("pass");
        let request = String::from("getblockchaininfo");

        let client = Client::new(url, user, pass);
        let res = client.send_request(request, vec![]);

        match res { 
            Ok(_) => assert!(true),
            Err(e) => assert!(false, format!("{:?}", e)),
        }
    }
}
