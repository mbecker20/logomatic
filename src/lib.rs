use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;
use serde_derive::Serialize;

pub use serde_json::json;

#[derive(Serialize)]
pub struct LogMessage<T: Serialize, D: Serialize> {
    pub ts: i64,
    #[serde(rename = "type")]
    pub log_type: T,
    pub data: D,
}

fn create_log_msg<T: Serialize, D: Serialize>(log_type: T, data: D) -> LogMessage<T, D> {
    LogMessage {
        ts: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64,
        log_type,
        data,
    }
}

pub fn log(log_type: impl Serialize, data: impl Serialize) {
    println!("{}", create_log_msg(log_type, data));
}

pub fn log_err(log_type: impl Serialize, data: impl Serialize) {
    eprintln!("{}", create_log_msg(log_type, data));
}

pub fn log_json(log_type: impl Serialize, data: impl Serialize) {
    println!(
        "{}",
        serde_json::to_string(&create_log_msg(log_type, data)).unwrap()
    );
}

pub fn log_err_json(log_type: impl Serialize, data: impl Serialize) {
    eprintln!(
        "{}",
        serde_json::to_string(&create_log_msg(log_type, data)).unwrap()
    );
}

impl<T: Serialize, D: Serialize> Display for LogMessage<T, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | {} | {}",
            self.ts,
            serde_json::to_string(&self.log_type).unwrap(),
            serde_json::to_string(&self.data).unwrap()
        )
    }
}
