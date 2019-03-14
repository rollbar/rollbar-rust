use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::configuration::Configuration;
use crate::types::Item;
use reqwest::Client;

const QUEUE_DEPTH: usize = 50;

pub trait Transport: Send + Sync + 'static {
    fn send(&self, item: Item);

    fn shutdown(&self, timeout: Duration) -> bool {
        let _timeout = timeout;
        true
    }
}

pub struct HttpTransport {
    sender: Mutex<SyncSender<Option<Item>>>,
    queue_depth: Arc<Mutex<usize>>,
    signal: Arc<Condvar>,
    shutdown: Arc<AtomicBool>,
    _handle: Option<JoinHandle<()>>,
}

impl HttpTransport {
    pub fn new(configuration: &Configuration) -> Self {
        let (tx, rx) = sync_channel(QUEUE_DEPTH);
        let signal = Arc::new(Condvar::new());
        let shutdown = Arc::new(AtomicBool::new(false));
        let client = Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(configuration.timeout))
            .build()
            .unwrap();
        #[allow(clippy::mutex_atomic)]
        let queue_depth = Arc::new(Mutex::new(0));
        let endpoint = configuration.endpoint.clone();
        let _handle = Some(spawn_sender(
            endpoint,
            client,
            rx,
            queue_depth.clone(),
            signal.clone(),
            shutdown.clone(),
        ));
        let sender = Mutex::new(tx);
        HttpTransport {
            sender,
            queue_depth,
            signal,
            shutdown,
            _handle,
        }
    }
}

impl Transport for HttpTransport {
    fn send(&self, item: Item) {
        *self.queue_depth.lock().unwrap() += 1;
        if self.sender.lock().unwrap().try_send(Some(item)).is_err() {
            *self.queue_depth.lock().unwrap() -= 1;
        }
    }

    fn shutdown(&self, timeout: Duration) -> bool {
        debug!("http transport shutdown");
        {
            let guard = self.queue_depth.lock().unwrap();
            if *guard == 0 {
                if let Ok(sender) = self.sender.lock() {
                    sender.send(None).ok();
                }
                return true;
            } else if let Ok(sender) = self.sender.lock() {
                match sender.try_send(None) {
                    Err(TrySendError::Full(_)) => {}
                    Ok(_) | Err(_) => {
                        return self.signal.wait_timeout(guard, timeout).is_ok();
                    }
                }
            }
        }
        if let Ok(sender) = self.sender.lock() {
            sender.send(None).ok();
        }
        let guard = self.queue_depth.lock().unwrap();
        self.signal.wait_timeout(guard, timeout).is_ok()
    }
}

impl Drop for HttpTransport {
    fn drop(&mut self) {
        debug!("http transport drop");
        self.shutdown.store(true, Ordering::SeqCst);
        if let Ok(sender) = self.sender.lock() {
            sender.send(None).ok();
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
    rx: Receiver<Option<Item>>,
    queue_depth: Arc<Mutex<usize>>,
    signal: Arc<Condvar>,
    shutdown: Arc<AtomicBool>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        while let Some(item) = rx.recv().unwrap_or(None) {
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
                .send()
            {
                Ok(mut resp) => {
                    let resp: Option<Response> = resp.json().ok();
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
    })
}
