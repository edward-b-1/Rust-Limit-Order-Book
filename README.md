# Rust-Limit-Order-Book

This repository contains an implementation of a Limit Order Book in Rust. The Limit Order Book is used to merge 3 Order Books from 3 Crypto Exchanges, and calculate the dollar cost of buying and dollar profit from selling 10 BTC.

# How to run it:

```
cargo run --bin rust-limit-order-book
```

There are also tests which can be run using `cargo test`.

# Design

There is a rather unusual design decision in that floating point encoded price values are used as a key for data structures. This would usually be a cause for concern due to the effect of precision loss in floating point calculations. However, in this particular implementation, some additional restrictions are imposed on the operations which can be performed on floats.

Normally, in production grade software, prices would be encoded using either fixed point values, some other type from a libary which implements decimal encoding, or (possibly) strings.

In this particular case, the risk of loss in precision is mitigated as no arithmatic calculations are performed on the floating point price values before they are used as a key to a data structure.

The series of operations performed on incoming data is as follows:

- Incoming price values are encoded as strings
- These string values are parsed and converted to floating point values
- The price values are small enough to be encoded as floating point values without loss of precision. In other words, there are less than 64 bits of information in each price string, so no information is lost
- Since the same function is used to parse every string value, it is guaranteed that for any input string, the same output float is always generated

I chose this design for two reasons:

1. It demonstrates a detailed understanding of how floating point encoding works
2. I personally thought it was more interesting than choosing fixed point or other BCD type. It provides more to discuss during an interview

The IEEE 745 standard defines how floating point values are encoded. For a 64 bit floating point value, the first bit represents the sign (S), the following 11 bits are used for the exponent (E), and the final 52 bits are used for the mantissa (M).

The value encoded is calculated by the formula

```
(-1)^S * (1 + M * 2^(-52)) * 2^(E-127)
```

assuming that the mantissa (M) is an integer value in the range 0 to 2^52 - 1.

Some numbers, like 100, can be encoded exactly using a 64 bit floating point representation.

- 100 = (1 + 1/2 + 1/16) * 2^6

Others, such as 0.1, cannot. The closest possible representation of 0.1 as a 64 bit floating point value is

```
0.1000000000000000055511151231257827021181583404541015625
```

or, in binary (S, E, M)

```
0 01111111011 1001100110011001100110011001100110011001100110011010
```

You can play with IEEE 754 numbers here:

- https://www.h-schmidt.net/FloatConverter/IEEE754.html

# Example Output:

```
Program start
Coinbase URL: https://api.exchange.coinbase.com/products/BTC-USD/book?level=2
Coinbase Response Status Code: 200 OK
Gemini URL: https://api.gemini.com/v1/book/btcusd
Gemini Response Status Code: 200 OK
cKraken Response Status Code: 200 OK
Total cost to BUY 10 BTC: $551430.0568316274
Total profit from SELL 10 BTC: $551374.7803173161
Total cost to BUY 10 BTC by source exchange: {"COINBASE": 26077.470196010403, "GEMINI": 0.0, "KRAKEN": 525352.586635617}
Total profit from SELL 10 BTC by source exchange: {"COINBASE": 26077.470196010403, "GEMINI": 0.0, "KRAKEN": 525352.586635617}
Total volume BUY by source exchange: {"COINBASE": 16236896.332188558, "GEMINI": 30.869781560000003, "KRAKEN": 159.87999999999994}
Total volume SELL by source exchange: {"COINBASE": 7563.39447730015, "GEMINI": 67.43406581999999, "KRAKEN": 97.87300000000005}
Round Trip Cost (10 BTC): 55.27651431132108
Spread (All Exchanges): Some(-5.180000000000291)
Spreads: {"COINBASE": Some(3.5499999999956344), "GEMINI": Some(4.92000000000553), "KRAKEN": Some(0.09999999999854481)}
Highest Bid, Lowest Ask by Exchange:
Bids: {"COINBASE": 55134.76, "GEMINI": 55143.49, "KRAKEN": 55142.9}
Asks: {"COINBASE": 55138.31, "GEMINI": 55148.41, "KRAKEN": 55143.0}
Program ends
```

There is a delay between each REST request. Each REST request is made in a synchronous, blocking manner. Sometimes this causes a negative spread to be shown across a pair of exchanges. Rather than being a real arbitrage opportunity, it is more likely caused by the market moving during the latency of each request.

Assessing whether this is a real arb opportunity would involve quite a complex analysis which would require some work on measing the time synchronization and latency to each exchange.
