mod client;

use std::{thread, time::Duration};

use client::Client;

#[derive(Debug)]
enum State {
    Growing,
    Reducing,
    Stable,
}

fn main() {
    let mut id = 0;
    let mut clients =  Vec::new();
    let mut state = State::Growing;

    loop {
        match state {
            State::Growing => {
                clients.push(Client::new(id, 150));
                id += 1;
            },
            State::Reducing => {
                if let Some(c) = clients.pop() {
                    c.join();
                }
            },
            State::Stable => break,
        }

        let failed = clients.iter().filter(|c| c.has_failed()).count();

        state = match state {
            State::Growing => if failed > 0 {State::Reducing} else {State::Growing},
            State::Reducing => if failed > 0 {State::Reducing} else {State::Stable},
            State::Stable => State::Stable,
        };
        
        println!("State: {:?} clients: {}", state, clients.len());
        thread::sleep(Duration::from_millis(10));
    }

    clients.into_iter().for_each(|c| c.join());
}