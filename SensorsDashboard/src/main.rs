mod sensor;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{thread,vec};

use sensor::start_sensor;
fn main() {
    let sensors = vec![
        "Temperature",
        "Humidity",
        "Pressure",
    ];

    let data = Arc::new(Mutex::new(HashMap::new()));

    for sensor in  &sensors {
        let sensor_data = Arc::clone(&data);
        let sensor_name = sensor.to_string();
        thread::spawn(|| {
            start_sensor(sensor_name, sensor_data);
        });
    }

    loop {
        thread::sleep(Duration::from_secs(5));
        let data_lock = data.lock().unwrap();

        print!("\n --- Sensor Data ---");

        for (sensor, value) in data_lock.iter() {
            println!("\n{}: {}", sensor, value);
        }
    }

}
