# xrpl

A multithreaded websocket client library for interacting with the xrp ledger's json rpc. The client maintains a single websocket connection, and is able to make atomic requests to the json rpc with a single function call, or make subscriptions with a function that returns a listener for incoming messages. Multiple Requests can be made in paralell, since each is processed in it's own thread. 

## Usage

To send a simple rpc command:

```rust
use xrpl::XrplClient;

#[tokio::main]
async fn main() {
    let client = XrplClient::new("wss://xrpl.ws").await.unwrap();
    let server_info = client.get_server_info().await.unwrap();

    println!("{server_info}");
}
```

To create a subscription:

```rust
use xrpl::XrplClient;

#[tokio::main]
async fn main() {
    let client = XrplClient::new("wss://xrpl.ws").await.unwrap();
    let mut listener = client.subscribe_to_ledger().await.unwrap();

    loop {
        let msg = listener.recv().await;
        println!("{msg}")
    }
}
```

