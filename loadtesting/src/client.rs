use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::Duration};

use websocket::{ClientBuilder, OwnedMessage};

pub struct Client {
    failed: Arc<Mutex<bool>>,
    thread: JoinHandle<()>,
}

impl Client {
    pub fn new(id: i32, speed_ms: i32) -> Client {
        let failed = Arc::new(Mutex::new(false));
        let failed_copy = failed.clone();
        Client {
            thread: thread::spawn(move || {
                let res = Client::run(id, failed_copy.clone(), speed_ms);
                if res.is_none() {
                    let _ = failed_copy.lock().map(|mut f| *f = true);
                }
            }),
            failed,
        }
    }
    pub fn has_failed(&self) -> bool {
        self.failed.lock().map(|f| *f).unwrap_or(true)
    }
    pub fn join(self) {
        let _ = self.failed.lock().map(|mut f| *f = true);
        let _ = self.thread.join();
    }
    fn run(id: i32, failed: Arc<Mutex<bool>>, speed_ms: i32) -> Option<()> {
        let mut client = ClientBuilder::new("ws://localhost:8080")
            .unwrap().connect(None).ok()?;

        let message = OwnedMessage::Text(format!(r#"{{"Connection": "l{}"}}"#, id).to_string());

        client.send_message(&message).ok()?;

        while !failed.lock().map(|f| *f).unwrap_or(false) {
            let messages = vec![
                OwnedMessage::Text(r#"{"ChangeDirection": "Right"}"#.to_string()),
                OwnedMessage::Text(r#"{"ChangeDirection": "Down"}"#.to_string()),
                OwnedMessage::Text(r#"{"ChangeDirection": "Left"}"#.to_string()),
                OwnedMessage::Text(r#"{"ChangeDirection": "Up"}"#.to_string()),
            ];

            for m in messages {
                client.send_message(&m).ok()?;
                thread::sleep(Duration::from_millis(speed_ms as u64));
            }
        };
        Some(())
    }
}