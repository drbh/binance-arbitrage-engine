use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn read_configure() -> Vec<String> {
    let data = fs::read_to_string("config.ttc").expect("Unable to read file");
    let mut pairs: Vec<String> = vec![];
    for line in data.lines() {
        let v: Vec<&str> = line.split("=").collect();
        pairs.push(v[1].to_string());
    }
    pairs
}

pub fn get_timestamp() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
}
