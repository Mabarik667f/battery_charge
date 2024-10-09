use std::fs;
use std::path;

pub struct Battery {
    capacity: u8,
    status: String,
}

pub struct Config {
    time_period: u8,
    min_lifetime_percentage: u8,
    notification: String,
}

impl Battery {
    pub fn new(path: &str) -> Battery {
        let capacity = get_battery_capacity(path);
        let status = get_battery_status(path);
        Battery { capacity, status }
    }
}

fn get_battery_capacity(path: &str) -> u8 {
    let abs_path = path::absolute(format!("{path}/capacity")).unwrap();
    fs::read_to_string(abs_path)
        .unwrap()
        .trim()
        .parse::<u8>()
        .unwrap()
}

fn get_battery_status(path: &str) -> String {
    let abs_path = path::absolute(format!("{path}/status")).unwrap();
    fs::read_to_string(abs_path).unwrap()
}

pub fn get_all_batteries() -> Vec<Battery> {
    let abs_path = path::absolute("/sys/class/power_supply/").unwrap();
    let mut batteries: Vec<Battery> = Vec::new();
    let mut c = 0;
    loop {
        let path_to_bat = format!("{}/BAT{}", abs_path.to_str().unwrap(), c);
        let p = &path_to_bat;
        let bat = path::absolute(p).unwrap();
        if !bat.exists() {
            println!("not Exists!");
            break;
        }
        batteries.push(Battery::new(p));
        println!("{bat:#?}");
        c += 1
    }
    batteries
}

fn get_total_capacity(batteries: Vec<Battery>) -> u8 {
    let mut total_cap = 0;
    for bat in batteries {
        total_cap += bat.capacity;
    }
    total_cap
}
