use battery_charge::{read_notification_data, run};
use cron::Schedule;
use std::process;
use std::str::FromStr;

fn main() {
    let config = match read_notification_data() {
        Ok(config) => config.config,
        Err(e) => {
            eprintln!("Problem with notification.toml args: {e}");
            process::exit(1);
        }
    };

    let exp = format!("0 */{} * * * *", config.time_period);
    let schedule = Schedule::from_str(&exp).unwrap();
    run(schedule, config)
}
