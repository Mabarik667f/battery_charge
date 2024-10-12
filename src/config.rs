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
