use std::{ops::Sub, sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::Duration};

use websocket::{ClientBuilder, OwnedMessage};

use chrono::Local;

pub struct Client {
    failed: Arc<Mutex<bool>>,
    thread: JoinHandle<()>,
    average: Arc<Mutex<Vec<i64>>>
}

impl Client {
    pub fn new(id: i32, speed_ms: i32, host: String) -> Client {
        let failed = Arc::new(Mutex::new(false));
        let failed_copy = failed.clone();
        let average = Arc::new(Mutex::new(Vec::new()));
        let average_copy = average.clone();
        Client {
            thread: thread::spawn(move || {
                let res = Client::run(id, failed_copy.clone(), speed_ms, average_copy, host.as_str());
                if res.is_none() {
                    let _ = failed_copy.lock().map(|mut f| *f = true);
                }
            }),
            failed,
            average
        }
    }
    pub fn has_failed(&self) -> bool {
        self.failed.lock().map(|f| *f).unwrap_or(true)
    }
    pub fn average(&self) -> Option<i64> {
        let list = self.average.lock().ok()?;
        if list.len() < 10 {
            None
        } else {
            Some(list.iter().sum::<i64>() / list.len() as i64)
        }
    }
    pub fn join(self) {
        let _ = self.failed.lock().map(|mut f| *f = true);
        let _ = self.thread.join();
    }
    fn run(id: i32, failed: Arc<Mutex<bool>>, speed_ms: i32, diffs: Arc<Mutex<Vec<i64>>>, host: &str) -> Option<()> {
        let mut client = ClientBuilder::new(host)
            .unwrap().connect(None).ok()?;

        let message = OwnedMessage::Text(format!(r#"{{"Connection": "l{}"}}"#, id).to_string());

        client.send_message(&message).ok()?;

        let _ = client.set_nonblocking(true);

        let mut last_time = Local::now().sub(Duration::from_millis(300));

        while !failed.lock().map(|f| *f).unwrap_or(false) {
            let messages = vec![
                OwnedMessage::Text(r#"{"ChangeDirection": "Right"}"#.to_string()),
                OwnedMessage::Text(r#"{"ChangeDirection": "Down"}"#.to_string()),
                OwnedMessage::Text(r#"{"ChangeDirection": "Left"}"#.to_string()),
                OwnedMessage::Text(r#"{"ChangeDirection": "Up"}"#.to_string()),
            ];

            for m in messages {
                client.send_message(&m).ok()?;

                for _ in 0..10 {
                    if let Ok(mut diffs) = diffs.lock() {
                        if let Ok(OwnedMessage::Text(message)) = client.recv_message() {
                            if message.starts_with(r#"{"ChangeInfos":"#) {
                                let now = Local::now();
                                let diff = now.signed_duration_since(&last_time).num_milliseconds();
                                diffs.push(diff);
                                last_time = now;
    
                                while diffs.len() > 50 {
                                    diffs.remove(0);
                                }
                            } else if message.starts_with(r#"{"Error":"#) {
                                return None;
                            }
                        }
                    }
                    thread::sleep(Duration::from_millis(speed_ms as u64/10));
                }

            }
        };
        Some(())
    }
}