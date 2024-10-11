use std::env;
use std::fs;
use std::path;

pub struct Config {
    pub time_period: u8,
    pub min_lifetime_percentage: u8,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let time_period = match args.next() {
            Some(arg) => match arg.parse::<u8>() {
                Ok(arg) => arg,
                _ => return Err("time_period Expected integer!"),
            },
            None => return Err("Didn't get a time_period minute time"),
        };

        let min_lifetime_percentage = match args.next() {
            Some(arg) => match arg.parse::<u8>() {
                Ok(arg) => arg,
                _ => return Err("min_lifetime_percentage Expected integer!"),
            },
            None => return Err("Didn't get a min_lifetime_percentage"),
        };

        Ok(Config {
            time_period,
            min_lifetime_percentage,
        })
    }
}

#[derive(Debug)]
pub struct Battery {
    capacity: u8,
    status: ChargingStatus,
}

#[derive(Debug)]
pub enum ChargingStatus {
    Charging,
    Notcharging,
}

impl Battery {
    pub fn new(path: &str) -> Battery {
        let capacity = get_battery_capacity(path);
        let status = get_battery_status(path);
        Battery { capacity, status }
    }
}

pub fn run() {}

fn get_battery_capacity(path: &str) -> u8 {
    let abs_path = path::absolute(format!("{path}/capacity")).unwrap();
    fs::read_to_string(abs_path)
        .unwrap()
        .trim()
        .parse::<u8>()
        .unwrap()
}

fn get_battery_status(path: &str) -> ChargingStatus {
    let abs_path = path::absolute(format!("{path}/status")).unwrap();
    match fs::read_to_string(abs_path)
        .unwrap()
        .replace(" ", "")
        .trim()
        .to_string()
        .as_str()
    {
        "Notcharging" | "Discharging" => ChargingStatus::Notcharging,
        "Charging" => ChargingStatus::Charging,
        _ => panic!("Undefined status!"),
    }
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
    println!("{batteries:#?}");
    batteries
}

pub fn get_data_for_batteries(batteries: Vec<Battery>) -> bool {
    let batteries_ref = &batteries;
    let total_cap = get_total_capacity(batteries_ref);
    let is_device_charging = any_battery_charging(batteries_ref);

    if total_cap > 20 || is_device_charging {
        false
    } else {
        true
    }
}

fn get_total_capacity(batteries: &Vec<Battery>) -> u8 {
    let mut total_cap = 0;
    let mut c = 0;
    for bat in batteries {
        total_cap += bat.capacity;
        c += 1;
    }
    total_cap / c
}

fn any_battery_charging(batteries: &Vec<Battery>) -> bool {
    for bat in batteries {
        match bat.status {
            ChargingStatus::Charging => return true,
            _ => continue,
        }
    }
    false
}
