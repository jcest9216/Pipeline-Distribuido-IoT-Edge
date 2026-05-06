use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use serde::Deserialize;

#[derive(Deserialize)]
struct EdgeResult {
    sensor_id: String,
    risk: f32,
    alert: bool,
}

fn handle_client(mut stream: TcpStream, global: Arc<Mutex<Vec<f32>>>) {
    let mut buffer = Vec::new();

    if stream.read_to_end(&mut buffer).is_ok() {
        if let Ok(data) = serde_json::from_slice::<EdgeResult>(&buffer) {
            let mut g = global.lock().unwrap();
            g.push(data.risk);

            let avg = g.iter().sum::<f32>() / g.len() as f32;

            println!(
                "Sensor {} | riesgo: {:.2} | promedio global: {:.2}",
                data.sensor_id, data.risk, avg
            );

            if data.alert {
                println!("Alerta de incendio");
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    let global = Arc::new(Mutex::new(Vec::new()));

    println!("Coordinador escuchando en 8000");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let g = global.clone();
            std::thread::spawn(|| handle_client(stream, g));
        }
    }
}

