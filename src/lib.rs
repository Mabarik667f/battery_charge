use std::fs;
use std::path;
use std::process::Command;
use std::thread;

use chrono::Utc;
use cron::Schedule;
use serde_derive::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct NotificationData {
    pub config: Config,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub title: String,
    pub text: String,
    pub time_period: u8,
    pub min_lifetime_percentage: u8,
}

pub fn read_notification_data() -> Result<NotificationData, &'static str> {
    let contents = match fs::read_to_string("notification.toml") {
        Ok(c) => c,
        Err(_) => {
            return Err("Could not read file");
        }
    };
    let data: NotificationData = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            return Err("Error load data for toml file");
        }
    };
    Ok(data)
}

#[derive(Debug)]
struct Battery {
    capacity: u8,
    status: ChargingStatus,
}

#[derive(Debug)]
enum ChargingStatus {
    Charging,
    Notcharging,
}

impl Battery {
    pub fn new(path: &str) -> Battery {
        Battery {
            capacity: get_battery_capacity(path),
            status: get_battery_status(path),
        }
    }
}

pub fn run(schedule: Schedule, config: Config) {
    let bats_paths = get_all_batteries_paths();
    loop {
        let batteries = get_batteries(&bats_paths);
        let battery_is_low = batteries_is_low(batteries, config.min_lifetime_percentage);
        if battery_is_low {
            let notification_data = vec![&config.title, &config.text];
            let _ = Command::new("notify-send").args(&notification_data).spawn();
        }
        let datetime = schedule.upcoming(Utc).next().unwrap();
        let until = datetime - Utc::now();
        thread::sleep(until.to_std().unwrap());
    }
}

fn get_battery_capacity(path: &str) -> u8 {
    let abs_path =
        path::absolute(format!("{path}/capacity")).expect("Capacity param not found this dir");
    fs::read_to_string(abs_path)
        .expect("Error reading file")
        .trim()
        .parse::<u8>()
        .expect("Conversion error to integer")
}

fn get_battery_status(path: &str) -> ChargingStatus {
    let abs_path =
        path::absolute(format!("{path}/status")).expect("Status param not found this dir");
    match fs::read_to_string(abs_path)
        .expect("Error reading file")
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

fn get_batteries(bats_paths: &Vec<String>) -> Vec<Battery> {
    let mut batteries: Vec<Battery> = Vec::new();
    for b_path in bats_paths {
        batteries.push(Battery::new(&b_path));
    }
    batteries
}

fn get_all_batteries_paths() -> Vec<String> {
    let abs_path =
        path::absolute("/sys/class/power_supply/").expect("Power supply param not found this dir");
    let mut batteries: Vec<String> = Vec::new();

    for dir in fs::read_dir(&abs_path).expect("Dir not found") {
        let word = dir.unwrap().file_name().into_string().unwrap();
        if word.contains("BAT") {
            let path_to_bat = format!("{}/{}", abs_path.to_str().unwrap(), word);
            batteries.push(path_to_bat);
        }
    }
    if batteries.len() == 0 {
        panic!("Batteries not found or batteries have custom names!");
    }
    batteries
}

fn batteries_is_low(batteries: Vec<Battery>, min_percentage: u8) -> bool {
    let batteries_ref = &batteries;
    let total_cap = get_total_capacity(batteries_ref);
    let is_device_charging = any_battery_charging(batteries_ref);

    if total_cap > min_percentage || is_device_charging {
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

#[cfg(test)]
mod tests {
    use super::*;

    struct TestContext {
        bat_low_not: Battery,
        bat_high_not: Battery,
        bat_low_ch: Battery,
        bat_high_ch: Battery,
    }

    fn setup() -> TestContext {
        TestContext {
            bat_low_not: Battery {
                capacity: 10,
                status: ChargingStatus::Notcharging,
            },
            bat_high_not: Battery {
                capacity: 80,
                status: ChargingStatus::Notcharging,
            },
            bat_low_ch: Battery {
                capacity: 10,
                status: ChargingStatus::Charging,
            },
            bat_high_ch: Battery {
                capacity: 80,
                status: ChargingStatus::Charging,
            },
        }
    }

    #[test]
    fn battery_low_notcharging_status() {
        let bats = setup();
        assert_eq!(any_battery_charging(&vec![bats.bat_low_not]), false);
    }

    #[test]
    fn battery_low_charging_status() {
        let bats = setup();
        assert_eq!(any_battery_charging(&vec![bats.bat_low_ch]), true);
    }
    #[test]
    fn batteries_low_one_charging_status() {
        let bats = setup();
        assert_eq!(
            any_battery_charging(&vec![bats.bat_low_not, bats.bat_low_ch]),
            true
        );
    }

    #[test]
    fn total_cap_one_battery() {
        let bats = setup();
        assert_eq!(get_total_capacity(&vec![bats.bat_high_ch]), 80)
    }

    #[test]
    fn total_cap_three_batteries() {
        let bats = setup();
        assert_eq!(
            get_total_capacity(&vec![bats.bat_high_ch, bats.bat_low_ch, bats.bat_high_not]),
            56
        );
    }

    #[test]
    fn batteries_is_low_one_charging() {
        let bats = setup();
        assert_eq!(
            batteries_is_low(vec![bats.bat_low_not, bats.bat_low_ch], 20),
            false
        );
    }

    #[test]
    fn batteries_is_low_no_charging() {
        let bats = setup();
        assert_eq!(
            batteries_is_low(vec![bats.bat_low_not, bats.bat_high_not], 50),
            true
        );
    }
}
