use crate::{configuration::Configuration, types::Item};

use futures::channel::{mpsc, oneshot};
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct HttpTransport {
    configuration: Configuration,
    client: Client,
    sender: Arc<Mutex<mpsc::Sender<Item>>>,
    send_shutdown: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

use super::Transport;

use std::time::Duration;

use futures::stream::StreamExt;

impl Transport for HttpTransport {
    fn send(&self, item: Item) {
        if let Ok(mut items) = self.sender.lock() {
            log::info!("sending: {:?}", item);

            if let Err(error) = items.try_send(item) {
                log::error!("error sending item: {}", error);
            }
        }
    }

    fn config(&self) -> &Configuration {
        &self.configuration
    }

    fn shutdown(&self, timeout: Duration) -> bool {
        if let Ok(mut so) = self.send_shutdown.lock() {
            if let Some(signal) = so.take() {
                if let Err(_) = signal.send(()) {
                    log::error!("error sending shutdown");

                    false
                } else {
                    true
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

use super::QUEUE_DEPTH;

impl HttpTransport {
    pub fn new(configuration: Configuration) -> Result<Self, reqwest::Error> {
        let mut client = Client::builder().build()?;

        let (sender, receiver) = mpsc::channel::<Item>(QUEUE_DEPTH);

        let (send_shutdown, receive_shutdown) = oneshot::channel();

        let mut transport = Self {
            configuration,
            client,
            sender: Arc::new(Mutex::new(sender)),
            send_shutdown: Arc::new(Mutex::new(Some(send_shutdown))),
        };

        transport.run(receiver, receive_shutdown);

        Ok(transport)
    }

    fn run(&mut self, mut items: mpsc::Receiver<Item>, mut shutdown: oneshot::Receiver<()>) {

        let client = self.client.clone();
        let conf = self.configuration.clone();

        spawn_local(async move {
            loop {
                futures::select! {
                    _ = shutdown => break,
                    item = items.next() => {
                        log::info!("-> {:?}", item);

                        if let Some(item) = item {
                            if let Err(error) = post(&client, &conf, &item).await {
                                log::info!("error sending request: {:?}", error);
                            } else {
                                log::info!("sent request: {:?}", item);
                            }
                        }
                    },
                };
            }
        });
    }
}

async fn post(client: &Client, conf: &Configuration, item: &Item) -> Result<(), Option<reqwest::Error>> {
    let access_token = conf.access_token.as_ref().map_or(Err(None), Ok)?;
    let endpoint = &conf.endpoint;

    client
        .post(endpoint.as_str())
        .json(&item)
        .header("X-Rollbar-Access-Token", access_token)
        .send()
        .await
        .map(|_| ())
        .map_err(Some)
}
