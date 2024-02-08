use std::{thread, time::Duration};

use websocket::{ClientBuilder, OwnedMessage};

fn main() {
    let threads: Vec<thread::JoinHandle<()>> = (0..100).into_iter().map(|i| {
        thread::spawn(move || {
            connect(i)
        })
    }).collect();

    threads.into_iter().for_each(|t| {t.join().unwrap();});
}

fn connect(id: i32) {
    let mut client = ClientBuilder::new("ws://localhost:8080")
        .unwrap().connect(None).unwrap();

    client.send_message(&OwnedMessage::Text(format!(r#"{{"Connection": "loadtest{}"}}"#, id).to_string())).unwrap();

    thread::sleep(Duration::from_millis(1000));

    client.send_message(&OwnedMessage::Close(None)).unwrap();
    client.shutdown().unwrap();
}