extern crate binance;
use binance::websockets::*;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

mod calculator;
use crate::calculator::check_triangle_trades;

fn read_configure() -> Vec<String> {
    let data = fs::read_to_string("config.ttc").expect("Unable to read file");
    let mut pairs: Vec<String> = vec![];
    for line in data.lines() {
        let v: Vec<&str> = line.split("=").collect();
        pairs.push(v[1].to_string());
    }
    pairs
}

fn main() {
    //
    // SETUP ZMQ CONNECTION
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .bind("tcp://127.0.0.1:5555")
        .expect("failed binding publisher");

    //
    // READ IN THE CONFIG AND SET VARS
    let pairs = read_configure();
    let pair_a = pairs[0].to_owned();
    let pair_b = pairs[1].to_owned();
    let pair_c = pairs[2].to_owned();
    println!("{} {} {}", pair_a, pair_b, pair_c);

    //
    // SET SOME TOP LEVEL VARIABLES DURING RUNTIME
    let mut btcusdt: f32 = 0.0 as f32;
    let mut ethusdt: f32 = 0.0 as f32;
    let mut ethbtc: f32 = 0.0 as f32;

    let mut money_a_time: u64 = 0.0 as u64;
    let mut money_b_time: u64 = 0.0 as u64;
    let mut money_c_time: u64 = 0.0 as u64;

    //
    //
    let agg_trade: String = format!("!ticker@arr");
    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| match event {
        WebsocketEvent::DayTicker(ticker_events) => {
            for tick_event in ticker_events {
                if tick_event.symbol == pair_a {
                    btcusdt = tick_event.current_close.parse().unwrap();
                    money_a_time = tick_event.event_time;
                }
                if tick_event.symbol == pair_b {
                    ethusdt = tick_event.current_close.parse().unwrap();
                    money_b_time = tick_event.event_time;
                }
                if tick_event.symbol == pair_c {
                    ethbtc = tick_event.current_close.parse().unwrap();
                    money_c_time = tick_event.event_time;
                }
            }

            // General: 0.1% trading fee
            let default_fee_percentage = 0.001;
            let start_amt = 1.0;

            let results =
                check_triangle_trades(btcusdt, ethusdt, ethbtc, start_amt, default_fee_percentage);
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let in_ms = since_the_epoch.as_secs() * 1000
                + since_the_epoch.subsec_nanos() as u64 / 1_000_000;

            let times: Vec<u64> = vec![money_a_time, money_b_time, money_c_time, in_ms];

            println!("{:?}", times);
            println!("{:?}", results);
        }

        _ => return,
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    web_socket.event_loop();
}
