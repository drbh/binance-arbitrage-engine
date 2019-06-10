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
                    // println!(
                    //     "{} {} Symbol: {}, price: {}, qty: {} avg: {}",
                    //     tick_event.event_type,
                    //     tick_event.event_time,
                    //     tick_event.symbol,
                    //     tick_event.best_bid,
                    //     tick_event.best_bid_qty,
                    //     tick_event.average_price
                    // );
                    btcusdt = tick_event.average_price.parse().unwrap();
                }
                if tick_event.symbol == "ETHUSDT" {
                    // println!(
                    //     "{} {} Symbol: {}, price: {}, qty: {} avg: {}",
                    //     tick_event.event_type,
                    //     tick_event.event_time,
                    //     tick_event.symbol,
                    //     tick_event.best_bid,
                    //     tick_event.best_bid_qty,
                    //     tick_event.average_price
                    // );

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

            let a = btcusdt; // one btc to usd
            let b = ethusdt; // on eth to usd
            let c = ethbtc; // on eth to btc

            // btcs to usdt
            let z = 1.0 * a;
            // usdt to eths
            let y = z / b;
            // eths to btc
            let x = y * c;

            let s = format!("{} {} {} {}", btcusdt, ethusdt, ethbtc, x);
            // println!("{:?}", s);
            publisher
                .send(&s, 0)
                .expect("failed sending first envelope");
        }

        _ => return,
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    web_socket.event_loop();
}
