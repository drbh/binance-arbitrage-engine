pub fn check_triangle_trades(
    a: f32,
    b: f32,
    c: f32,
    start_amt: f32,
    default_fee_percentage: f32,
) -> Vec<f32> {
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

    vec![
        convert_usd,
        usd_fee,
        remaining_usd,
        convert_eth,
        eth_fee,
        remaining_eth,
        convert_btc,
        btc_fee,
        remaining_btc,
    ]
}
