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

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::schema::posts;

use crate::models::{NewPost, Post};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(
    conn: &PgConnection,

    id: &'a f64,
    in_ms: &'a f64,
    money_a_time: &'a f64,
    money_d_time: &'a f64,
    money_e_time: &'a f64,
    convert_usd: &'a f64,
    usd_fee: &'a f64,
    remaining_usd: &'a f64,
    convert_eth: &'a f64,
    eth_fee: &'a f64,
    remaining_eth: &'a f64,
    convert_btc: &'a f64,
    btc_fee: &'a f64,
    remaining_btc: &'a f64,
) -> Post {
    let new_post = NewPost {
        id: in_ms,
        in_ms: in_ms,
        money_a_time: money_a_time,
        money_d_time: money_d_time,
        money_e_time: money_e_time,
        convert_usd: convert_usd,
        usd_fee: usd_fee,
        remaining_usd: remaining_usd,
        convert_eth: convert_eth,
        eth_fee: eth_fee,
        remaining_eth: remaining_eth,
        convert_btc: convert_btc,
        btc_fee: btc_fee,
        remaining_btc: remaining_btc,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
