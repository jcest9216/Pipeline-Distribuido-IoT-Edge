use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use rand::Rng;
use serde::Serialize;

#[derive(Serialize)]
struct SensorData {
    id: String,
    temp: f32,
    humidity: f32,
    smoke: f32,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let id = args.get(1).unwrap_or(&"sensor-1".to_string()).clone();

    // Indicar ip del edge
    let edge_addr = "10.0.0.2:7000";

    loop {
        let mut rng = rand::thread_rng();

        let data = SensorData {
            id: id.clone(),
            temp: rng.gen_range(20.0..50.0),
            humidity: rng.gen_range(10.0..90.0),
            smoke: rng.gen_range(0.0..1.0),
        };

        let json = serde_json::to_vec(&data).unwrap();

        match TcpStream::connect(&edge_addr) {
            Ok(mut stream) => {
                let _ = stream.write_all(&json);
                println!("{} envió datos", id);
            }
            Err(_) => {
                println!("{} no pudo conectar con edge", id);
            }
        }

        thread::sleep(Duration::from_secs(2));
    }
}

