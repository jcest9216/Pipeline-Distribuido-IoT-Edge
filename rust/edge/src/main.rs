use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SensorData {
    id: String,
    temp: f32,
    humidity: f32,
    smoke: f32,
}

#[derive(Serialize)]
struct EdgeResult {
    sensor_id: String,
    risk: f32,
    alert: bool,
}

fn calculate_risk(data: &SensorData) -> f32 {
    (data.temp * 0.5) + (data.smoke * 30.0) - (data.humidity * 0.3)
}

fn handle_client(mut stream: TcpStream, coord_addr: String) {
    let mut buffer = Vec::new();

    if stream.read_to_end(&mut buffer).is_ok() {
        if let Ok(data) = serde_json::from_slice::<SensorData>(&buffer) {
            
            let risk = calculate_risk(&data);

            let result = EdgeResult {
                sensor_id: data.id,
                risk,
                alert: risk > 50.0,
            };

            let json = serde_json::to_vec(&result).unwrap();

            match TcpStream::connect(&coord_addr) {
                Ok(mut coord) => {
                    let _ = coord.write_all(&json);
                    println!("Enviado al coordinador");
                }
                Err(e) => {
                    println!("Error conectando al coordinador: {}", e);
                }
            }

            println!("Procesado: riesgo = {}", result.risk);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let listen_addr = args.get(1).unwrap_or(&"0.0.0.0:7000".to_string()).clone();
    let coord_addr = args.get(2).unwrap_or(&"10.0.0.1:8000".to_string()).clone();

    let listener = TcpListener::bind(&listen_addr).unwrap();

    println!("Edge escuchando en {}", listen_addr);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let coord_clone = coord_addr.clone();
            std::thread::spawn(|| handle_client(stream, coord_clone));
        }
    }
}

