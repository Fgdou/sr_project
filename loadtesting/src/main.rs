mod client;

use std::{env, thread, time::Duration};

use charts::{Chart, LineSeriesView, ScaleLinear};
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

    let mut latency_by_clients = Vec::new();

    let host = match env::args().into_iter().nth(1) {
        Some(host) => host,
        None => panic!("Usage: ./loadtesting <host>"),
    };

    println!("Running with host {}", host);

    loop {        
        let count = clients.len();
        clients.retain(|c| !c.has_failed());
        
        let filtered = clients.iter()
            .filter_map(|c| c.average());
        let average = if filtered.clone().count() == 0 {0} else {
            filtered.clone().sum::<i64>() / filtered.count() as i64
        };
        let failed = count - clients.len();

        match state {
            State::Growing(_) => {
                clients.push(Client::new(id, 500, host.clone()));
                id += 1;

                if average > 1000 {
                    state = State::Stable{t: 0, target: clients.len() as i32};
                }
                let diff = Local::now().signed_duration_since(start);
                if diff.num_seconds() > 240 {
                    state = State::Stable { target: clients.len() as i32, t: 0 }
                }

                latency_by_clients.push((clients.len() as f32, average as f32));
            },
            State::Stable{target, t: _} => {
                while (clients.len() as i32) < target {
                    clients.push(Client::new(id, 150, host.clone()));
                }
            },
        }

        state = match state {
            State::Growing(n) => if n > 100 {State::Stable{target: clients.len() as i32, t: 0}} else {State::Growing(n + failed as i32 )},
            State::Stable{t: 200, target} => {
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

    let max_clients = latency_by_clients.iter().max_by(|x, y| x.0.partial_cmp(&y.0).unwrap()).unwrap().0;
    let max_latency = latency_by_clients.iter().max_by(|x, y| x.1.partial_cmp(&y.1).unwrap()).unwrap().1;

    let x = ScaleLinear::new()
        .set_domain(vec![0_f32, max_clients])
        .set_range(vec![0, 600-40-60]);

    let y = ScaleLinear::new()
        .set_domain(vec![300_f32, max_latency])
        .set_range(vec![600-90-50, 0]);

    let view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_visibility(false)
        .load_data(&latency_by_clients)
        .unwrap();

    Chart::new()
        .set_width(600)
        .set_height(600)
        .set_margins(90, 40, 50, 60)
        .add_title("Latency".to_string())
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Latency (ms)")
        .add_bottom_axis_label("# clients")
        .save("latency.svg")
        .unwrap();
}