# xrpl

A multithreaded websocket client library for interacting with the xrp ledger's json rpc. At its core, the client maintains a single websocket connection, but is able to make atomic requests to the json rpc with a single function call, or make subscriptions with a function that returns a listener for incoming messages. These features are implemented using the websocket implementation by `tokio-tungstenite`, as well as using channels implemented by `tokio::sync`.

## Usage

To send a simple rpc command:

```rust
use xrpl::XrplClient;

#[tokio::main]
async fn main() {
    let client = XrplClient::new("wss://xrpl.ws").await.unwrap();
    let server_info = client.get_server_info().await;
    println!("{server_info}");
}

```

To create a subscription:

```rust
use xrpl::XrplClient;

#[tokio::main]
async fn main() {
    let client = XrplClient::new("wss://xrpl.ws").await.unwrap();
    let mut listener = client.subscribe_to_ledger().await;

    loop {
        let msg = listener.recv().await;
        println!("{msg}")
    }
}

```
