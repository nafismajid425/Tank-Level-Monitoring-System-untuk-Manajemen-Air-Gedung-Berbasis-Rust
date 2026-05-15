# 🚰 Tank Level Monitoring System
### Sistem Monitoring Level Tangki Air Gedung Berbasis Rust

> **Evaluasi Tengah Semester (ETS) — Algoritma dan Pemrograman**
> Program Studi Teknik Instrumentasi | Semester Genap 2025/2026
> Institut Teknologi Sepuluh Nopember

---

## 👥 Tim Pengembang

| Nama | NRP |
|------|-----|
| Muflih Azhar | 2042251079 |
| M. Nafis Abdul Majid | 2042251126 |

**Dosen Pengampu:** Ahmad Radhy, S.Si., M.Si.
**Bahasa Pemrograman:** Rust

---

## 📋 Deskripsi Proyek

Sistem ini dirancang untuk memonitor level air pada tangki silinder secara otomatis melalui terminal. Program memproses data sensor mentah menggunakan filter **Simple Moving Average (SMA)** dan **kalibrasi mandiri** untuk menjaga akurasi pembacaan, lalu secara otomatis mengendalikan status pompa berdasarkan ambang batas yang telah ditentukan.

---

## ⚙️ Parameter Desain

| Parameter | Nilai |
|-----------|-------|
| Objek Ukur | Tangki Silinder, radius `r = 2.0 m` |
| Batas Bawah / Low Level (LL) | `2.0 meter` → Pompa **NYALA** |
| Batas Atas / High-High (HH) | `9.0 meter` → Pompa **MATI** |
| Galat Sensor (error) | `0.5 meter` (dikoreksi via software) |
| Window Size SMA | `3 data terakhir` |

---

## 🔄 Algoritma & Flowchart

Sistem berjalan secara interaktif mengikuti alur logika berikut:

```
           ┌─────────────┐
           │    Mulai    │
           └──────┬──────┘
                  │
           ┌──────▼──────────────┐
      ┌───►│  Input Raw Level    │
      │    └──────┬──────────────┘
      │           │
      │    ┌──────▼──────────────────────┐
      │    │  Moving Average & Kalibrasi │
      │    │  havg = (hi + hi-1 + hi-2)/3│
      │    │  hfinal = havg + error      │
      │    └──────┬──────────────────────┘
      │           │
      │    ┌──────▼──────────┐    Ya   ┌──────────────────┐
      │    │  Level ≤ LL?    ├────────►│   Pompa NYALA    │
      │    └──────┬──────────┘         └────────┬─────────┘
      │           │ Tidak                       │
      │    ┌──────▼──────────┐    Ya   ┌────────▼─────────┐
      │    │  Level ≥ HH?    ├────────►│   Pompa MATI     │
      │    └──────┬──────────┘         └────────┬─────────┘
      │           │ Tidak                       │
      │    ┌──────▼──────────┐                  │
      │    │  Pompa STANDBY  │                  │
      │    └──────┬──────────┘                  │
      │           │                             │
      │    ┌──────▼──────────────────────────────┐
      │    │          Output Terminal             │
      │    └──────┬──────────────────────────────┘
      │           │
      └───────────┘ (loop kembali)
```

**Alur Logika:**
1. Menerima input pembacaan sensor mentah (raw level) dari pengguna via terminal
2. Menghitung **Simple Moving Average** dari 3 data terakhir untuk mereduksi noise
3. Menambahkan nilai galat `0.5 m` sebagai **kalibrasi mandiri**
4. Menghitung **volume air** menggunakan rumus silinder: `V = π × r² × hfinal`
5. Menentukan **status pompa** berdasarkan batas LL dan HH

---

## 🏗️ Struktur Program (OOP dengan Rust)

```
sistem_sensor/
├── src/
│   └── main.rs       # Source utama program
├── Cargo.toml        # Konfigurasi project Rust
├── Cargo.lock
└── .gitignore
```

Program mengimplementasikan konsep **Object-Oriented Programming (OOP)** dengan dua struct utama:

```
MonitoringSystem
├── sensor: Sensor          ← enkapsulasi data & metode sensor
│   ├── name: String
│   ├── readings: Vec<f32>  ← buffer window SMA
│   ├── window_size: usize
│   └── calibration_error: f32
├── low_limit: f32          ← batas LL (2.0 m)
└── high_limit: f32         ← batas HH (9.0 m)
```

---

## 🔢 Komputasi Numerik

**1. Simple Moving Average (SMA)**

$$h_{avg} = \frac{h_i + h_{i-1} + h_{i-2}}{3}$$

**2. Kalibrasi Mandiri**

$$h_{final} = h_{avg} + error$$

**3. Perhitungan Volume Silinder**

$$V = \pi \times r^2 \times h_{final}$$

---

## 💻 Source Code

```rust
use std::io::{self, Write};

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

    fn calculate_volume(&self, height: f32) -> f32 {
        let radius: f32 = 2.0;
        let pi = 3.14159;
        pi * radius * radius * height
    }

    fn run_process(&mut self, raw_val: f32) {
        let processed = self.sensor.get_processed_level(raw_val);
        let volume = self.calculate_volume(processed);

        let status_pompa = if processed <= self.low_limit {
            "NYALA [>>>>] - Mengisi air"
        } else if processed >= self.high_limit {
            "MATI [____] - Tangki penuh"
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
        print!("Masukkan pembacaan sensor (meter): ");
        io::stdout().flush().unwrap();

        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Gagal membaca input");

        let trimmed = input_text.trim();
        if trimmed == "exit" {
            println!("Program berhenti. Sampai jumpa!");
            break;
        }

        match trimmed.parse::<f32>() {
            Ok(val) => {
                system.run_process(val);
                println!("--------------------------------------------------\n");
            }
            Err(_) => {
                println!("Input tidak valid! Harap masukkan angka.\n");
            }
        };
    }
}
```

---

## 🚀 Cara Menjalankan

**Prasyarat:** Pastikan [Rust](https://www.rust-lang.org/tools/install) sudah terinstal.

```bash
# Clone / masuk ke direktori project
cd sistem_sensor

# Build project
cargo build

# Jalankan program
cargo run
```

**Contoh interaksi terminal:**
```
=== SISTEM MONITORING TANGKI INTERAKTIF ===
Batas Bawah: 2m | Batas Atas: 9m
Ketik 'exit' untuk berhenti.

Masukkan pembacaan sensor (meter): 1.5
--------------------------------------------------
DATA SENSOR  | Raw: 1.50m | Final: 2.00m
VOLUME AIR   | 25.13 m³
STATUS POMPA | NYALA [>>>>] - Mengisi air
--------------------------------------------------
```

---

## 🧪 Hasil Pengujian

| No | Input Sensor | Rata-rata SMA | Ketinggian Final | Volume (m³) | Status Pompa |
|----|-------------|---------------|-----------------|-------------|--------------|
| 1  | 1.50 m      | 1.50 m        | 2.00 m          | 25.13       | ✅ NYALA (LL Tercapai) |
| 2  | 5.00 m      | 5.00 m        | 5.50 m          | 69.12       | ⏸️ STANDBY |
| 3  | 8.50 m      | 8.50 m        | 9.00 m          | 113.10      | 🛑 MATI (HH Tercapai) |

---

## 📊 Analisis

- **Filter SMA** dengan window 3 data terbukti efektif mereduksi noise sensor
- **Kalibrasi +0.5 m** mengimbangi deviasi sistematis sensor secara akurat
- **Kendali pompa** aktif tepat pada batas fisik LL = 2.0 m dan HH = 9.0 m
- **Perhitungan volume** memberikan informasi kapasitas tangki secara real-time

---

## ✅ Kesimpulan

Sistem monitoring level tangki interaktif ini berhasil mengintegrasikan:
- **OOP** — melalui struct `Sensor` dan `MonitoringSystem` dengan enkapsulasi penuh
- **Algoritma Kendali** — logika keputusan pompa berbasis threshold LL dan HH
- **Komputasi Numerik** — Simple Moving Average dan kalibrasi mandiri
- **Geometri Silinder** — perhitungan volume air secara presisi

Semua dikembangkan menggunakan bahasa pemrograman **Rust** yang aman dan efisien.

---

<div align="center">

**Departemen Teknik Instrumentasi**
Institut Teknologi Sepuluh Nopember · 2025/2026

</div>
