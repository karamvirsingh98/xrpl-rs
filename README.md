# xrpl

A multithreaded websocket client library for interacting with the xrp ledger's json rpc. At its core, the client maintains a single websocket connection, but is able to make atomic requests to the json rpc with a single function call, or make subscriptions with a function that returns a listener for incoming messages. These features are implemented using the websocket implementation by `tokio-tungstenite`, as well as using channels implemented by `tokio::sync`.

## Roadmap

- connect to the ledger, and make basic rpc requests ✅
  - open and maintain a websocket connection to the ledger ✅
  - make atomic json rpc requests for info ✅
  - make subscription requests with dedicated listeners ✅
- implement wallets
  - create a new wallet from entropy
  - derive a wallet from seed
- implement transactions
  - create typesafe transaction builder
  - sign transactions with wallet

## Usage

This client library was written to be as close to the javascript client as possible. Using the JavaScript client, you might make requests as such:

```js
import { Client } from "xrpl";

async function main() {
  const client = new Client("wss://xrpl.ws");
  const server_info = await client.request({ command: "server_info" });
  console.log(server_info);
}
```

Using this client, the same request may be performed as

```rust
use xrpl::XrplClient;

#[tokio::main]
async fn main() {
    let client = XrplClient::new("wss://xrpl.ws").await.unwrap();
    let server_info = client.get_server_info().await;
    println!("{server_info}");
}

```

Principally, the difference is that instead of a single request method to which you pass the rpc call object (as in the JavaScript client), each rpc call has been wrapped in a function in this one. That being said, under the hood each of these does resolve to a single request method, however the differences between how TypeScript and Rust handle enums makes it difficult to have inferred type outputs which are variants of an enum.

There is some significant variation in how subscriptions are handled, however. Since we do not have JavaScript's EventEmitter API at our disposal, managing subscription streams requires a different approach to the api.

For example, if you would like to subscribe to the ledger stream in the JavaScript client, it would look something like this:

```js
import { Client } from "xrpl";

async function main() {
  const client = new Client("wss://xrpl.ws");
  await client.request({ command: "subscribe", streams: ["ledger"] });
  client.on("ledgerClosed", (msg) => console.log(msg));
}
```

Using this client, the same procedure would look something like this:

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

To overcome the lack of JavaScript's flexibility, these subscription listeners are implemented `tokio::mpsc::broadcast` channels. The listeners may be arbitrarily cloned and sent into new threads if needed, and so on. Moreover, the listners will only return messages of the correct type, based on subscription request made. So, subscribing to the orderbook stream for example returns a handle that only returns messages relaing to that orderbook. This way, you can have multiple threads each managing an individual subscription, though all are carried over a single socket.
