use battery_charge::{get_all_batteries, get_data_for_batteries, Config};
use chrono::Utc;
use cron::Schedule;
use std::env;
use std::process;
use std::process::Command;
use std::str::FromStr;
use std::thread;
use toml_parser::read_notification_data;

pub mod toml_parser;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let batteries = get_all_batteries();
    let a = get_data_for_batteries(batteries);
    println!("{a}");

    let notification_data = read_notification_data().unwrap();
    let args = vec![
        notification_data.config.title,
        notification_data.config.text,
    ];
    let exp = format!("0 */{} * * * *", config.time_period);
    let schedule = Schedule::from_str(&exp).unwrap();
    loop {
        let datetime = schedule.upcoming(Utc).next().unwrap();
        let until = datetime - Utc::now();
        thread::sleep(until.to_std().unwrap());
        let e = Command::new("notify-send").args(&args).spawn();
        println!("ERRORS: {e:#?}");
    }
}
