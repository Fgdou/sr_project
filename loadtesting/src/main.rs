mod client;

use std::{env, thread, time::Duration};

use chrono::Local;
use client::Client;

#[derive(Debug, PartialEq)]
enum State {
    Growing(i32),
    Stable{target: i32, t: i32},
}

fn main() {
    let mut id = 100;
    let mut clients: Vec<Client> =  Vec::new();
    let mut state = State::Growing(0);
    let start = Local::now();

    let host = match env::args().into_iter().nth(1) {
        Some(host) => host,
        None => panic!("Usage: ./loadtesting <host>"),
    };

    println!("Running with host {}", host);

    loop {        
        let count = clients.len();
        clients.retain(|c| !c.has_failed());
        
        let average = if clients.is_empty() {0} else {
            clients.iter()
                .filter_map(|c| c.average())
                .sum::<i64>() / clients.len() as i64
        };
        let failed = count - clients.len();

        match state {
            State::Growing(_) => {
                clients.push(Client::new(id, 500, host.clone()));
                id += 1;

                if average > 500 {
                    state = State::Stable{t: 0, target: clients.len() as i32};
                }
                let diff = Local::now().signed_duration_since(start);
                if diff.num_seconds() > 60 {
                    state = State::Stable { target: clients.len() as i32, t: 0 }
                }
            },
            State::Stable{target, t: _} => {
                while (clients.len() as i32) < target {
                    clients.push(Client::new(id, 150, host.clone()));
                }
            },
        }

        state = match state {
            State::Growing(n) => if n > 100 {State::Stable{target: clients.len() as i32, t: 0}} else {State::Growing(n + failed as i32 )},
            State::Stable{t: 500, target} => {
                println!("{} players {} ms", target, average);

                break
            },
            State::Stable{target, t} => State::Stable{target, t: t+1},
        };
        
        println!("State: {:?} clients: {} Ping: {}ms", state, clients.len(), average);
        thread::sleep(Duration::from_millis(100));
    }

    println!("Stopping");
    clients.into_iter().for_each(|c| c.join());
}