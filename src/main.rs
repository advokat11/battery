use std::io;
use std::fs;
use std::env;
use chrono::prelude::*;
use battery::{Manager};

fn main() {
    let manager = Manager::new().unwrap();
    let battery = manager.batteries().unwrap().next().unwrap().unwrap();
    let mut charge_percent: f32;

    loop {
        println!("Please enter the desired battery charge percentage (0-100):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        charge_percent = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if charge_percent >= 0.0 && charge_percent <= 100.0 {
            break;
        }
    }

    charge_percent /= 100.0;
    let state_of_charge = battery.state_of_charge().value;

    if state_of_charge >= charge_percent {
        let now: DateTime<Local> = Local::now();
        let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let desktop_path = env::var("USERPROFILE").unwrap() + r"\Desktop\battery.txt";
        fs::write(desktop_path, formatted_time).expect("Unable to write file");
    }
}
