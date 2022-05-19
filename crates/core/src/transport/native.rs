use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

use tokio::{sync::mpsc, runtime::Runtime};

use crate::configuration::Configuration;
use crate::types::Item;

use reqwest::{Client, Proxy};

#[derive(Debug)]
pub struct HttpTransport {
    sender: Mutex<mpsc::Sender<Option<Item>>>,
    queue_depth: Arc<Mutex<usize>>,
    signal: Arc<Condvar>,
    shutdown: Arc<AtomicBool>,
    configuration: Arc<Configuration>,
    runtime: Option<Runtime>,
}

impl HttpTransport {
    pub fn new(configuration: Configuration) -> Result<Self, std::io::Error> {
        let (tx, rx) = mpsc::channel(super::QUEUE_DEPTH);
        let signal = Arc::new(Condvar::new());
        let shutdown = Arc::new(AtomicBool::new(false));

        let mut client_builder = Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(configuration.timeout));

        let proxy = configuration
            .proxy
            .as_ref()
            .and_then(|proxy| Proxy::all(proxy).ok());

        if let Some(proxy) = proxy {
            client_builder = client_builder.proxy(proxy);
        }

        if let Some(proxy) = &configuration.proxy {
            client_builder = client_builder.proxy(Proxy::all(proxy).unwrap());
        }

        let client = client_builder.build().unwrap();
        #[allow(clippy::mutex_atomic)]
        let queue_depth = Arc::new(Mutex::new(0));
        let endpoint = configuration.endpoint.clone();
        let runtime = Runtime::new()?;

        spawn_sender(
            endpoint,
            client,
            rx,
            queue_depth.clone(),
            signal.clone(),
            shutdown.clone(),
            &runtime,
        );

        let sender = Mutex::new(tx);

        Ok(HttpTransport {
            sender,
            queue_depth,
            signal,
            shutdown,
            configuration: Arc::new(configuration),
            runtime: Some(runtime),
        })
    }
}

use super::Transport;

impl Transport for HttpTransport {
    fn send(&self, item: Item) {
        *self.queue_depth.lock().unwrap() += 1;
        if self.sender.lock().unwrap().try_send(Some(item)).is_err() {
            *self.queue_depth.lock().unwrap() -= 1;
        }
    }

    fn config(&self) -> &Configuration {
        &self.configuration
    }

    fn shutdown(&self, timeout: Duration) -> bool {
        if let Some(runtime) = &self.runtime {
            runtime.block_on(async {
                debug!("http transport shutdown");
                {
                    let guard = self.queue_depth.lock().unwrap();
                    if *guard == 0 {
                        if let Ok(sender) = self.sender.lock() {
                            sender.send(None).await.ok();
                        }
                        return true;
                    } else if let Ok(sender) = self.sender.lock() {
                        match sender.send(None).await {
                            Err(_) => {}
                            Ok(_) => {
                                return self.signal.wait_timeout(guard, timeout).is_ok();
                            }
                        }
                    }
                }
                if let Ok(sender) = self.sender.lock() {
                    sender.send(None).await.ok();
                }
                let guard = self.queue_depth.lock().unwrap();
                self.signal.wait_timeout(guard, timeout).is_ok()
            })
        } else {
            false
        }
    }
}

impl Drop for HttpTransport {
    fn drop(&mut self) {
        debug!("http transport drop");
        self.shutdown.store(true, Ordering::SeqCst);
        if let Some(runtime) = &self.runtime {
            runtime.block_on(async {
                if let Ok(sender) = self.sender.lock() {
                    sender.send(None).await.ok();
                }
            });
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    err: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Success>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Success {
    id: Option<String>,
    uuid: Option<String>,
}

fn spawn_sender(
    endpoint: String,
    client: Client,
    mut rx: mpsc::Receiver<Option<Item>>,
    queue_depth: Arc<Mutex<usize>>,
    signal: Arc<Condvar>,
    shutdown: Arc<AtomicBool>,
    runtime: &Runtime,
) {
    runtime.spawn(async move {
        while let Some(item) = rx.recv().await {
            let item = match item.clone() {
                Some(item) => item,
                None => break
            };

            if shutdown.load(Ordering::SeqCst) {
                let mut size = queue_depth.lock().unwrap();
                *size = 0;
                signal.notify_all();
                break;
            }

            let access_token = item.access_token.as_str();

            match client
                .post(endpoint.as_str())
                .json(&item)
                .header("X-Rollbar-Access-Token", access_token)
                .send().await
            {
                Ok(resp) => {
                    let resp = resp.json().await;
                    let resp: Option<Response> = resp.ok();
                    if let Some(r) = resp {
                        info!("Item sent\n{}", serde_json::to_string_pretty(&r).unwrap());
                    } else {
                        info!("Error deserializing response, but successfully sent item");
                    }
                }
                Err(err) => {
                    info!("Failed to send item: {}", err);
                }
            }

            let mut size = queue_depth.lock().unwrap();
            *size -= 1;
            if *size == 0 {
                signal.notify_all();
            }
        }
        info!("send thread shutdown!");
    });
}
