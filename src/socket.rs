use std::{fmt::Debug, time::Duration};

use anyhow::Context;
use futures_util::{SinkExt, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use tokio::{
    sync::{
        broadcast::{self, Receiver},
        mpsc, oneshot,
    },
    time,
};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use uuid::Uuid;

use crate::types::{
    request::{XrplRequest, XrplSubscriptionRequest},
    response::XrplResponse,
};

pub struct XrplSocket {
    res_receiver: broadcast::Receiver<String>,
    req_sender: mpsc::Sender<String>,
}

impl XrplSocket {
    pub async fn new(url: &str) -> anyhow::Result<XrplSocket> {
        let (res_sender, res_receiver) = broadcast::channel(10);
        let (req_sender, mut req_receiver) = mpsc::channel(10);

        let client = XrplSocket {
            res_receiver,
            req_sender,
        };

        let (stream, _) = connect_async(url)
            .await
            .context("failed to open websocket stream")?;

        let (mut sender, mut receiver) = stream.split();

        // spawn the thread for receiving messages from the webscoket, and sending them over the res_sender broadcast channel
        tokio::spawn(async move {
            loop {
                let msg = receiver.next().await;
                if let Some(msg) = msg {
                    if let Err(e) = &msg {
                        eprintln!("got error message over websocket - \n {e:?}")
                    }

                    let msg = msg.unwrap();
                    match msg {
                        Message::Text(msg) => {
                            let send_res = res_sender.send(msg.to_string());
                            if let Err(e) = send_res {
                                eprintln!(
                                    "error sending websocket response over mpsc channel - \n {e:?}"
                                )
                            }
                        }
                        _ => {}
                    }
                }
            }
        });

        // spawn the thread for receiving messages from the mpsc channel, and sending them over the websocket sender, or pinging once every five seconds›
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    msg = req_receiver.recv() => {
                        if let Some(msg) = msg {
                            let send_res = sender.send(Message::Text(msg)).await;
                            if let Err(e) = send_res {
                                eprintln!("error sending request message - \n {e:?}");
                            }
                        }
                    }
                    _ = time::sleep(Duration::from_secs(5)) => {
                            let ping_res = sender.send(Message::Ping(Vec::new())).await;
                            if let Err(e) = ping_res {
                                eprintln!("failed to ping socket - \n {e}");
                            }
                    }
                }
            }
        });

        Ok(client)
    }

    pub async fn request<T: Clone + Send + Debug + Serialize + DeserializeOwned + 'static>(
        &self,
        req: XrplRequest,
    ) -> anyhow::Result<XrplResponse<T>> {
        let sender = self.req_sender.clone();
        let mut receiver = self.res_receiver.resubscribe();
        let (out_sender, out_rec) = oneshot::channel::<XrplResponse<T>>();

        tokio::spawn(async move {
            let req_id = Uuid::new_v4().to_string();
            let req = req.to_string(&req_id);
            let send_res = sender.send(req).await;
            if let Err(e) = send_res {
                eprintln!("error sending request - \n {e:?}");
            }
            loop {
                let msg = receiver.recv().await;
                if let Err(e) = msg {
                    eprintln!("got error message from broadcast receiver - \n {e:?}");
                    continue;
                }
                let msg = msg.unwrap();
                let response = serde_json::from_str::<XrplResponse<T>>(&msg);

                if let Err(_) = &response {
                    continue;
                }

                let response = response.unwrap();
                let id = &response.id;
                if &req_id == id {
                    let res = out_sender.send(response);
                    if let Err(e) = res {
                        eprintln!("error sending message on out channel - \n {e:?}");
                    }
                    break;
                }
            }
        });

        out_rec
            .await
            .context("failed to get response msg from out channel")
    }

    pub async fn subscribe<T: Clone + Send + Debug + Serialize + DeserializeOwned + 'static>(
        &self,
        req: XrplSubscriptionRequest,
    ) -> anyhow::Result<Receiver<T>> {
        let sender = self.req_sender.clone();
        let mut receiver = self.res_receiver.resubscribe();
        let (res_sender, res_receiver) = broadcast::channel::<T>(10);

        tokio::spawn(async move {
            let send_res = sender.send(req.to_string()).await;
            if let Err(e) = send_res {
                eprintln!("error sending request - \n {e:?}");
            }
            loop {
                let msg = receiver.recv().await;
                if let Err(e) = msg {
                    eprintln!("got error message from broadcast receiver - \n {e:?}");
                    continue;
                }

                let msg = msg.unwrap();
                let serialized = serde_json::from_str::<T>(&msg);
                if let Err(_) = serialized {
                    continue;
                }

                let send_res = res_sender.send(serialized.unwrap());
                if let Err(e) = send_res {
                    eprintln!("error sending subscription message out of thread - \n {e:?}")
                }
            }
        });

        Ok(res_receiver)
    }
}
