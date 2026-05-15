use std::io::{self, Write}; // Diperlukan untuk membaca input dari keyboard

struct Sensor {
    name: String,
    readings: Vec<f32>,
    window_size: usize,
    calibration_error: f32,
}

impl Sensor {
    fn new(name: &str, window_size: usize, error: f32) -> Self {
        Self {
            name: name.to_string(),
            readings: Vec::new(),
            window_size,
            calibration_error: error,
        }
    }

    fn moving_average(&mut self, new_value: f32) -> f32 {
        self.readings.push(new_value);
        if self.readings.len() > self.window_size {
            self.readings.remove(0);
        }
        let sum: f32 = self.readings.iter().sum();
        sum / self.readings.len() as f32
    }

    fn calibrate(&self, value: f32) -> f32 {
        value + self.calibration_error
    }
    
    fn get_processed_level(&mut self, raw_value: f32) -> f32 {
        let averaged = self.moving_average(raw_value);
        self.calibrate(averaged)
    }
}

struct MonitoringSystem {
    sensor: Sensor,
    low_limit: f32,
    high_limit: f32,
}

impl MonitoringSystem {
    fn new(ll: f32, hh: f32, err: f32) -> Self {
        Self {
            sensor: Sensor::new("Water Level", 3, err),
            low_limit: ll,
            high_limit: hh,
        }
    }

    // Fungsi menghitung volume silinder (V = pi * r^2 * t)
    fn calculate_volume(&self, height: f32) -> f32 {
        let radius: f32 = 2.0; // Asumsi jari-jari tangki 2 meter
        let pi = 3.14159;
        pi * radius * radius * height
    }

    fn run_process(&mut self, raw_val: f32) {
        let processed = self.sensor.get_processed_level(raw_val);
        let volume = self.calculate_volume(processed);
        
        let status_pompa = if processed <= self.low_limit {
            "NYALA [>>>>] - Mengisi air"
        } else if processed >= self.high_limit {
            "MATI  [____] - Tangki penuh"
        } else {
            "STANDBY [----]"
        };

        println!("--------------------------------------------------");
        println!("DATA SENSOR  | Raw: {:.2}m | Final: {:.2}m", raw_val, processed);
        println!("VOLUME AIR   | {:.2} m³", volume);
        println!("STATUS POMPA | {}", status_pompa);
    }
}

fn main() {
    let mut system = MonitoringSystem::new(2.0, 9.0, 0.5); 
    
    println!("=== SISTEM MONITORING TANGKI INTERAKTIF ===");
    println!("Batas Bawah: {}m | Batas Atas: {}m", system.low_limit, system.high_limit);
    println!("Ketik 'exit' untuk berhenti.\n");

    loop {
        // 1. Menampilkan prompt input
        print!("Masukkan pembacaan sensor (meter): ");
        io::stdout().flush().unwrap(); // Memastikan teks muncul sebelum input

        // 2. Membaca input dari keyboard
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Gagal membaca input");

        // 3. Logika untuk keluar dari program
        let trimmed = input_text.trim();
        if trimmed == "exit" {
            println!("Program berhenti. Sampai jumpa!");
            break;
        }

        // 4. Mengubah teks input menjadi angka f32
        match trimmed.parse::<f32>() {
            Ok(val) => {
                // 5. Menjalankan proses monitoring
                system.run_process(val);
                println!("--------------------------------------------------\n");
            }
            Err(_) => {
                println!("Input tidak valid! Harap masukkan angka.\n");
            }
        };
    }
}