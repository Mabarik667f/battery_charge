use serde_derive::Deserialize;
use std::fs;
use toml;

#[derive(Debug, Deserialize)]
pub struct NotificationData {
    pub config: Notification,
}

#[derive(Debug, Deserialize)]
pub struct Notification {
    pub title: String,
    pub text: String,
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
    println!("{data:#?}");
    Ok(data)
}
