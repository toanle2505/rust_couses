use std::{collections::HashMap, sync::{Arc, Mutex}, thread};
use rand::Rng;

pub fn start_sensor(sensor: String,shared_data: Arc<Mutex<HashMap<String, f64>>>) {
    let mut rng = rand::thread_rng();
    loop {
        let value = match sensor.as_str() {
            "Temperature" => rng.gen_range(-10.0..35.0),
            "Humidity" => rng.gen_range(20.0..80.0),
            "Pressure" => rng.gen_range(950.0..1050.0),
            _ => 0.0,
        };

        {
            let mut  data_lock = shared_data.lock().unwrap();  
            data_lock.insert(sensor.clone(), value);
        }

        thread::sleep(std::time::Duration::from_secs(1));
    }
}