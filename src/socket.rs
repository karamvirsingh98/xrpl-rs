use std::time::Duration;

use anyhow::{anyhow, Context};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::{
    sync::{broadcast, mpsc, oneshot},
    time,
};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_util::sync::CancellationToken;

use crate::request::XrplSubscription;

pub struct XrplSocket {
    receiver: broadcast::Receiver<String>,
    sender: mpsc::Sender<String>,
    timeout_dur: Option<i64>,
    cancel: CancellationToken,
}

impl XrplSocket {
    pub async fn new(url: &str, timeout_dur: Option<i64>) -> anyhow::Result<XrplSocket> {
        let (receiver_out, receiver) = broadcast::channel(1000);
        let (sender, mut sender_in) = mpsc::channel(1000);

        let client = XrplSocket {
            receiver,
            sender,
            timeout_dur,
            cancel: CancellationToken::new(),
        };

        let (stream, _) = connect_async(url)
            .await
            .context("failed to open websocket stream")?;

        let (mut ws_sender, mut ws_receiver) = stream.split();

        // receive messages from the ws receiver, and send them over broadcast sender
        let cancel = client.cancel.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    msg = ws_receiver.next() => {
                        match msg {
                            Some(msg) => match msg {
                                Ok(msg) => match msg {
                                    Message::Text(msg) => {
                                        let send_res = receiver_out.send(msg);
                                        if let Err(e) = send_res {
                                            eprintln!("error sending websocket response over mpsc channel - \n {e:?}")
                                        }
                                    }
                                    _ => {}
                                },
                                Err(e) => {
                                    eprintln!("got error message over websocket - \n {e:?}");
                                    cancel.cancel();
                                }
                            },
                            None => {
                                eprintln!("got none message over websocket");
                                cancel.cancel();
                            }
                        };
                    }
                    _ = time::sleep(Duration::from_secs(15)) => {
                        eprintln!("socket timed out");
                        cancel.cancel();
                }
                    _ = cancel.cancelled() => {
                        break;
                    }
                }
            }
        });

        // receive message from the mpsc receiver, send them over ws sender, or ping once every five seconds.
        let cancel = client.cancel.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    msg = sender_in.recv() => {
                        match msg {
                            Some(msg) => {
                                let res = ws_sender.send(Message::Text(msg)).await;
                                match res {
                                    Ok(()) => {}
                                    Err(e) => {
                                        eprintln!("error sending request message - \n {e:?}");
                                        cancel.cancel();
                                    }
                                }
                            }
                            None =>{}
                        }
                    }
                    _ = time::sleep(Duration::from_secs(5)) => {
                            let ping_res = ws_sender.send(Message::Ping(Vec::new())).await;
                            if let Err(e) = ping_res {
                                eprintln!("failed to ping socket - \n {e}");
                                cancel.cancel();
                            }
                    }
                    _ = cancel.cancelled() => {
                        break;
                    }
                }
            }
        });

        Ok(client)
    }

    pub async fn request(&self, request: Value) -> anyhow::Result<String> {
        let cancel = self.cancel.clone();

        let mut ws_receiver = self.receiver.resubscribe();
        let (out_sender, out_rec) = oneshot::channel::<String>();

        let send_res = self.sender.send(request.to_string()).await;
        if let Err(e) = send_res {
            return Err(anyhow!(e));
        }

        tokio::spawn(async move {
            let req = request.as_object().unwrap();
            let req_id = req.get("id").unwrap();

            loop {
                if cancel.is_cancelled() {
                    break;
                }
                match ws_receiver.recv().await {
                    Ok(msg) => {
                        let response = serde_json::from_str::<Value>(&msg);
                        match response {
                            Ok(response) => {
                                let id = response.as_object().unwrap().get("id").unwrap();
                                if req_id == id {
                                    out_sender.send(response.to_string()).unwrap();
                                    break;
                                }
                            }
                            Err(e) => {
                                eprintln!("{e}");
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        break;
                    }
                }
            }
        });

        tokio::select! {
            res = out_rec => res.context("failed to get message from out sender"),
            _ = tokio::time::sleep(Duration::from_millis(self.timeout_dur.unwrap_or(5000) as u64)) => Err(anyhow!("request timed out"))
        }
    }

    pub async fn subscribe<T: XrplSubscription>(&self) -> broadcast::Receiver<T::Message> {
        let cancel = self.cancel.clone();
        let mut ws_receiver = self.receiver.resubscribe();

        let (sender, receiver) = broadcast::channel::<T::Message>(100);

        tokio::spawn(async move {
            loop {
                if cancel.is_cancelled() {
                    break;
                }
                match ws_receiver.recv().await {
                    Ok(msg) => {
                        let parsed = serde_json::from_str::<T::Message>(&msg);
                        match parsed {
                            Ok(parsed) => {
                                let res = sender.send(parsed);
                                if let Err(e) = res {
                                    eprintln!("{e}");
                                    break;
                                }
                            }
                            Err(e) => {
                                eprintln!("{e}");
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{e:#?}");
                        break;
                    }
                }
            }
        });

        receiver
    }
}
