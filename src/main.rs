extern crate binance;

// use binance::api::*;
// use binance::userstream::*;
use binance::websockets::*;

fn main() {
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .bind("tcp://127.0.0.1:5555")
        .expect("failed binding publisher");

    let agg_trade: String = format!("!ticker@arr");

    let mut btcusdt: f32 = "0".parse().unwrap();
    let mut ethusdt: f32 = "0".parse().unwrap();
    let mut ethbtc: f32 = "0".parse().unwrap();

    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| match event {
        WebsocketEvent::DayTicker(ticker_events) => {
            for tick_event in ticker_events {
                if tick_event.symbol == "BTCUSDT" {
                    btcusdt = tick_event.average_price.parse().unwrap();
                }
                if tick_event.symbol == "ETHUSDT" {
                    ethusdt = tick_event.average_price.parse().unwrap();
                }
                if tick_event.symbol == "ETHBTC" {
                    // println!(
                    //     "{} {} Symbol: {}, price: {}, qty: {} avg: {}",
                    //     tick_event.event_type,
                    //     tick_event.event_time,
                    //     tick_event.symbol,
                    //     tick_event.best_bid,
                    //     tick_event.best_bid_qty,
                    //     tick_event.average_price
                    // );
                    ethbtc = tick_event.average_price.parse().unwrap();
                }
            }

            // General: 0.1% trading fee

            let default_fee_percentage = 0.001;

            let a = btcusdt; // one btc to usd
            let b = ethusdt; // on eth to usd
            let c = ethbtc; // on eth to btc

            let start_amt = 1.0;
            // btcs to usdt
            let convert_usd = start_amt * a;

            let usd_fee = convert_usd * default_fee_percentage;

            let remaining_usd = convert_usd - usd_fee;
            // usdt to eths
            let convert_eth = remaining_usd / b;

            let eth_fee = convert_eth * default_fee_percentage;

            let remaining_eth = convert_eth - eth_fee;
            // eths to btc
            let convert_btc = remaining_eth * c;

            let btc_fee = convert_btc * default_fee_percentage;

            let remaining_btc = convert_btc - btc_fee;

            let s = format!(
                "{{
    \"btcusdt\": {} ,
    \"ethusdt\": {} ,
    \"ethbtc\": {} ,

    \"start_amt\": {} ,
    \"convert_usd\": {} ,
    \"usd_fee\": {} ,
    \"remaining_usd\": {} ,
    \"convert_eth\": {} ,
    \"eth_fee\": {} ,
    \"remaining_eth\": {} ,
    \"convert_btc\": {} ,
    \"btc_fee\": {} ,
    \"remaining_btc\": {} 
    
}}",
                btcusdt,
                ethusdt,
                ethbtc,
                start_amt,
                convert_usd,
                usd_fee,
                remaining_usd,
                convert_eth,
                eth_fee,
                remaining_eth,
                convert_btc,
                btc_fee,
                remaining_btc
            );

            let _execution = format!(
                "[
{{
    \"ticker\": \"BTCUSDT\",
    \"amount\": 1,
    \"price\": {},
    \"action\": \"SELL\"
}},
{{
    \"ticker\": \"ETHUSDT\",
    \"amount\": {},
    \"price\": {},
    \"action\": \"BUY\"
}},
{{
    \"ticker\": \"ETHBTC\",
    \"amount\": {},
    \"price\": {},
    \"action\": \"SELL\"
}},]",
                btcusdt, remaining_usd, ethusdt, remaining_eth, ethbtc,
            );

            // println!("{:?}", s);
            publisher
                .send(&s, 0)
                // .send(&s, 0)
                .expect("failed sending first envelope");

            if remaining_btc > 1.01 {
                publisher
                    .send(&_execution, 0)
                    // .send(&s, 0)
                    .expect("failed sending first envelope");
            }
        }

        _ => return,
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    web_socket.event_loop();
}
