# binance-arbitrage-engine


In terminal A:
```bash
cargo run
```

In terminal B:
```bash
go run client.go 
```

# What I know about fees
Fee are forsure `0.1% or 0.001` this is provide via Binance docs. But there are many ways to lower these fees - most notably by using BNB. This should be an option in the code - but is not at the moment.


A way to calculate - this website has it explained pretty well

"Binance charges a flat 0.1% fee for executing trades on their exchange, regardless of whether you're buying or selling and using limit or market orders for transactions. The flat fee is applied automatically once a buy or sell order is fulfilled, and is deducted from the end cryptocurrency.

For example, If you bought 1,000 TRX using ETH as the base currency, the 0.1% fee of 1 TRX will be applied and automatically deducted from your order, netting you 999 TRX. Conversely, if you sold all your XLM holdings for exactly 1 BTC, the 0.1% fee of 0.001 BTC will be applied and net you .999 BTC.

To show it in dollar terms, if you bought 10,000 TRX using BTC as the base currency with the going rate of $0.10 per TRX and $10,000 per BTC, you would pay 0.1 BTC ($1,000). Binance will then apply the 0.1% fee in TRX, which would come out to 10 TRX ($1.00), and net you 9,990 TRX ($999).

To turn the tables around, if you exchanged 10,000 TRX for BTC at the same going rate as above, you'd get 0.1 BTC ($1,000). The 0.1% fee would then be applied in BTC, which would come out to 0.0001 BTC ($1.00), netting you 0.0999 BTC ($999). In essence, Binance's flat 0.1% fee means that you'll get charged 1 coin per 1,000, 10 per 10,000, 100 per 100,000, and so on, regardless of which currency you buy or sell." [link](https://smartphones.gadgethacks.com/how-to/binance-101-fees-fine-print-you-need-know-before-trading-bitcoins-other-cryptocurrencies-0182067/)

# Data this program provides

#### Market Activity Stream
```JSON
{
    "btcusdt": 7855.3535,
    "ethusdt": 240.30733,
    "ethbtc": 0.03067585,

    "start_amt": 1,
    "convert_usd": 7855.3535,
    "usd_fee": 7.855354,
    "remaining_usd": 7847.498,
    "convert_eth": 32.65609,
    "eth_fee": 0.032656092,
    "remaining_eth": 32.623432,
    "convert_btc": 1.0007515,
    "btc_fee": 0.0010007515,
    "remaining_btc": 0.99975073 
    
}
```

#### Execution Engine Stream
```JSON
[
{
	"ticker": "BTCUSDT",
	"amount": 1,
	"price": 7855.3535,
	"action": "SELL"
},
{
	"ticker": "ETHUSDT",
	"amount": 7847.498,
	"price": 240.30733,
	"action": "BUY"
},
{
	"ticker": "ETHBTC",
	"amount": 32.623432,
	"price": 0.03067585,
	"action": "SELL"
},]
```