use bitcoind_request::{
    client::Client as BitcoindRequestClient, client::Request as BitcoindRequestRequest,
};
use jsonrpc::{serde_json::value::RawValue, simple_http, Response as JsonRPCResponse};

pub struct Client {
    pub bitcoind_request_client: BitcoindRequestClient,
}

pub struct Request<'a>(BitcoindRequestRequest<'a>);

impl<'a> Request<'a> {}

impl Client {
    // TODO: Add error handling if this fails
    pub fn new(url: &str, user: &str, pass: &str) -> Result<Self, simple_http::Error> {
        let bitcoind_request_client =
            BitcoindRequestClient::new(url, user, pass).expect("failed to create client");
        let client = Client {
            bitcoind_request_client,
        };
        Ok(client)
    }
    pub fn build_request<'a>(
        &self,
        command: &'a str,
        params: &'a Vec<Box<RawValue>>,
    ) -> Request<'a> {
        let bitcoind_request: BitcoindRequestRequest =
            self.bitcoind_request_client.build_request(command, &params);
        let request = Request(bitcoind_request);
        request
    }
    pub fn send_request(&self, request: Request) -> Result<JsonRPCResponse, jsonrpc::Error> {
        let response = self.bitcoind_request_client.send_request(request.0);
        response
    }
}
