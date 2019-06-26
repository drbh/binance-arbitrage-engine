extern crate binance;
use binance::websockets::*;
use std::collections::HashMap;
use std::time::Instant;
extern crate bytevec;
use bytevec::ByteEncodable;

mod calculator;
use crate::calculator::check_triangle_trades;

mod helper;
use crate::helper::get_timestamp;
use crate::helper::read_configure;

use crate::helper::create_post;
use crate::helper::establish_connection;

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

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
    let pair_d = pairs[3].to_owned();
    let pair_e = pairs[4].to_owned();
    let pair_f = pairs[5].to_owned();
    let pair_g = pairs[6].to_owned();
    let pair_h = pairs[7].to_owned();
    let pair_i = pairs[8].to_owned();

    println!("{:?}", pairs);

    //
    // SET SOME TOP LEVEL VARIABLES DURING RUNTIME
    let mut btcusdt: f32 = 0.0 as f32;

    let mut ethusdt: f32 = 0.0 as f32;
    let mut ethbtc: f32 = 0.0 as f32;

    let mut bnbusdt: f32 = 0.0 as f32;
    let mut bnbbtc: f32 = 0.0 as f32;

    let mut ltcusdt: f32 = 0.0 as f32;
    let mut ltcbtc: f32 = 0.0 as f32;

    let mut eosusdt: f32 = 0.0 as f32;
    let mut eosbtc: f32 = 0.0 as f32;

    let mut money_a_time: u64 = 0.0 as u64;
    let mut money_b_time: u64 = 0.0 as u64;
    let mut money_c_time: u64 = 0.0 as u64;
    let mut money_d_time: u64 = 0.0 as u64;
    let mut money_e_time: u64 = 0.0 as u64;
    let mut money_f_time: u64 = 0.0 as u64;
    let mut money_g_time: u64 = 0.0 as u64;
    let mut money_h_time: u64 = 0.0 as u64;
    let mut money_i_time: u64 = 0.0 as u64;

    // General: 0.1% trading fee
    let default_fee_percentage = 0.001;
    let start_amt = 1.0;
    // should add flag for math flip
    // or rev traverse triangle

    // Testing save speeds

    let connection = establish_connection();

    // let post = create_post(
    //     &connection,
    //     &0.0,
    //     &0.0,
    //     &0.0,
    //     &0.0,
    //     &0.0,
    //     &0.0,
    //     &0.0,
    //     &0.0,
    //     &0.0,
    //     &0.0,
    //     &0.0,
    //     &0.0,
    //     &0.0,
    //     &0.0,
    // );
    // println!("\nSaved draft with id {:?}", post.id);

    //
    //
    let agg_trade: String = format!("!ticker@arr");
    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| match event {
        WebsocketEvent::DayTicker(ticker_events) => {
            let _start = Instant::now();

            let mut contacts = HashMap::new();
            let mut already_found = 0;

            for tick_event in ticker_events {
                if tick_event.symbol == pair_a {
                    btcusdt = tick_event.current_close.parse().unwrap();
                    money_a_time = tick_event.event_time;
                    already_found += 1;
                }
                // ETH
                if tick_event.symbol == pair_b {
                    ethusdt = tick_event.current_close.parse().unwrap();
                    money_b_time = tick_event.event_time;
                    already_found += 1;
                }
                if tick_event.symbol == pair_c {
                    ethbtc = tick_event.current_close.parse().unwrap();
                    money_c_time = tick_event.event_time;
                    already_found += 1;
                }
                // BNB
                if tick_event.symbol == pair_d {
                    bnbusdt = tick_event.current_close.parse().unwrap();
                    money_d_time = tick_event.event_time;
                    already_found += 1;
                }
                if tick_event.symbol == pair_e {
                    bnbbtc = tick_event.current_close.parse().unwrap();
                    money_e_time = tick_event.event_time;
                    already_found += 1;
                }
                // LTC
                if tick_event.symbol == pair_f {
                    ltcusdt = tick_event.current_close.parse().unwrap();
                    money_f_time = tick_event.event_time;
                    already_found += 1;
                }
                if tick_event.symbol == pair_g {
                    ltcbtc = tick_event.current_close.parse().unwrap();
                    money_g_time = tick_event.event_time;
                    already_found += 1;
                }
                // EOS
                if tick_event.symbol == pair_h {
                    eosusdt = tick_event.current_close.parse().unwrap();
                    money_h_time = tick_event.event_time;
                    already_found += 1;
                }
                if tick_event.symbol == pair_i {
                    eosbtc = tick_event.current_close.parse().unwrap();
                    money_i_time = tick_event.event_time;
                    already_found += 1;
                }

                // allow early break
                if already_found == 9 {
                    break;
                }
            }

            let results =
                check_triangle_trades(btcusdt, ethusdt, ethbtc, start_amt, default_fee_percentage);

            let results2 =
                check_triangle_trades(btcusdt, bnbusdt, bnbbtc, start_amt, default_fee_percentage);

            let results3 =
                check_triangle_trades(btcusdt, ltcusdt, ltcbtc, start_amt, default_fee_percentage);

            let results4 =
                check_triangle_trades(btcusdt, eosusdt, eosbtc, start_amt, default_fee_percentage);

            let in_ms = get_timestamp();

            let times: Vec<u64> = vec![in_ms, money_a_time, money_b_time, money_c_time];
            let times_bytes = times.encode::<u8>().unwrap();

            let times2: Vec<u64> = vec![in_ms, money_a_time, money_d_time, money_e_time];
            let times2_bytes = times2.encode::<u8>().unwrap();

            let times3: Vec<u64> = vec![in_ms, money_a_time, money_f_time, money_g_time];
            let times3_bytes = times3.encode::<u8>().unwrap();

            let times4: Vec<u64> = vec![in_ms, money_a_time, money_h_time, money_i_time];
            let times4_bytes = times4.encode::<u8>().unwrap();

            let results_bytes = results.encode::<u8>().unwrap();
            let results2_bytes = results2.encode::<u8>().unwrap();
            let results3_bytes = results3.encode::<u8>().unwrap();
            let results4_bytes = results4.encode::<u8>().unwrap();

            contacts.insert("time", times_bytes);
            contacts.insert("value", results_bytes);

            contacts.insert("time2", times2_bytes);
            contacts.insert("value2", results2_bytes);

            contacts.insert("time3", times3_bytes);
            contacts.insert("value3", results3_bytes);

            contacts.insert("time4", times4_bytes);
            contacts.insert("value4", results4_bytes);

            let post = create_post(
                &connection,
                &(in_ms as f64),
                &(in_ms as f64),
                &(money_a_time as f64),
                &(money_b_time as f64),
                &(money_c_time as f64),
                &(results[0] as f64),
                &(results[1] as f64),
                &(results[2] as f64),
                &(results[3] as f64),
                &(results[4] as f64),
                &(results[5] as f64),
                &(results[6] as f64),
                &(results[7] as f64),
                &(results[8] as f64),
            );
            println!("\nSaved draft with id {:?}", post.id);

            let duration = _start.elapsed();
            println!("{} DONE: {:?}\n", in_ms, duration);
        }

        _ => return,
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    web_socket.event_loop();
}
