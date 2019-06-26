// use self::schema::schema;

// use crate::schema;
use crate::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id: f64,
    pub in_ms: f64,
    pub money_a_time: f64,
    pub money_d_time: f64,
    pub money_e_time: f64,
    pub convert_usd: f64,
    pub usd_fee: f64,
    pub remaining_usd: f64,
    pub convert_eth: f64,
    pub eth_fee: f64,
    pub remaining_eth: f64,
    pub convert_btc: f64,
    pub btc_fee: f64,
    pub remaining_btc: f64,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub id: &'a f64,
    pub in_ms: &'a f64,
    pub money_a_time: &'a f64,
    pub money_d_time: &'a f64,
    pub money_e_time: &'a f64,
    pub convert_usd: &'a f64,
    pub usd_fee: &'a f64,
    pub remaining_usd: &'a f64,
    pub convert_eth: &'a f64,
    pub eth_fee: &'a f64,
    pub remaining_eth: &'a f64,
    pub convert_btc: &'a f64,
    pub btc_fee: &'a f64,
    pub remaining_btc: &'a f64,
}
