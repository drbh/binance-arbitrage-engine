
// 100 times slower
// crossbeam::scope(|scope| {
//     for tick_event in &ticker_events {
//         let _pair_a = pair_a.clone();
//         let _pair_b = pair_b.clone();
//         let _pair_c = pair_c.clone();

//         scope.spawn(move |_| {
//             if tick_event.symbol == _pair_a {
//                 btcusdt = tick_event.current_close.parse().unwrap();
//                 money_a_time = tick_event.event_time;
//             }
//             if tick_event.symbol == _pair_b {
//                 ethusdt = tick_event.current_close.parse().unwrap();
//                 money_b_time = tick_event.event_time;
//             }
//             if tick_event.symbol == _pair_c {
//                 ethbtc = tick_event.current_close.parse().unwrap();
//                 money_c_time = tick_event.event_time;
//             }
//         });
//     }
// })
// .unwrap();

//

// extern crate crossbeam;
// crossbeam = "0.7.1"


 let s = format!(
        "{{
    \"{}\": {},
    \"{}\": {},
    \"{}\": {},
    \"start_amt\": {},
    \"convert_usd\": {},
    \"usd_fee\": {},
    \"remaining_usd\": {},
    \"convert_eth\": {},
    \"eth_fee\": {},
    \"remaining_eth\": {},
    \"convert_btc\": {},
    \"btc_fee\": {},
    \"remaining_btc\": {} 
    
}}",
        pair_a,
        btcusdt,
        pair_b,
        ethusdt,
        pair_c,
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
    \"ticker\": {},
    \"amount\": 1,
    \"price\": {},
    \"action\": \"SELL\"
}},
{{
    \"ticker\": {},
    \"amount\": {},
    \"price\": {},
    \"action\": \"BUY\"
}},
{{
    \"ticker\": {},
    \"amount\": {},
    \"price\": {},
    \"action\": \"SELL\"
}},]",
        pair_a, btcusdt, pair_b, remaining_usd, ethusdt, pair_c, remaining_eth, ethbtc,
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
    };