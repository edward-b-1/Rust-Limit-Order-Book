


// order book:
//
// key by: symbol (BTC)
//         order side
//         price level
//         queue per price level
//
// order information:
//
// ticker
// order side
// price
// volume
// source exchange (expect a maximum of 1 per price level)

// NOTE: Why floats (`f64`) for price?
//
// While it is true that we cannot expect to perform math operations on floating point
// numbers and obtain exact results, eg
//
//     1.0 + 2.0 != 3.0
//
// we *can* expect to consistently parse string representations of floats and obtain
// the same values.
//
// Since this entire codebase only performs string to floating point conversion
// operations when working with prices, and the floating point values are always of
// the same precision (`f64`) this is ok.
//
// NotNan<f64> provides a convenient way of recovering total ordering behaviour of
// floats.
//
// The alternative would be to use a library which implements fixed point numbers
// or Decimal types.


pub mod coinbase_lib;
pub mod gemini_lib;
pub mod kraken_lib;
pub mod limit_order_book_lib;
pub mod cost_functions;

use coinbase_lib::get_api_coinbase;
use coinbase_lib::dump_coinbase_response_text_to_file;
use coinbase_lib::load_coinbase_book;

use gemini_lib::get_api_gemini;
use gemini_lib::dump_gemini_response_text_to_file;
use gemini_lib::load_gemini_book;

use kraken_lib::get_api_kraken;
use kraken_lib::dump_kraken_response_text_to_file;
use kraken_lib::load_kraken_book;

use cost_functions::cost_function_buy;
use cost_functions::profit_function_sell;
use cost_functions::cost_function_buy_with_source_exchange;
use cost_functions::profit_function_sell_with_source_exchange;

use limit_order_book_lib::MultiTickerLimitOrderBook;
use limit_order_book_lib::OrderSide;

use ordered_float::NotNan;


fn main() {
    println!("Program start");

    let mut limit_order_book = MultiTickerLimitOrderBook::new();
    
    #[allow(non_snake_case)]
    let ticker_BTC_USD = "BTCUSD";
    let source_exchange_coinbase = "COINBASE";
    let source_exchange_gemini = "GEMINI";
    let source_exchange_kraken = "KRAKEN";

    let client = reqwest::blocking::Client::new();
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36";

    let response_coinbase_text = get_api_coinbase(&client, user_agent);
    dump_coinbase_response_text_to_file(&response_coinbase_text);
    load_coinbase_book(
        response_coinbase_text,
        ticker_BTC_USD,
        source_exchange_coinbase,
        &mut limit_order_book,
    );

    let response_gemini_text = get_api_gemini(&client, user_agent);
    dump_gemini_response_text_to_file(&response_gemini_text);
    load_gemini_book(
        response_gemini_text,
        ticker_BTC_USD,
        source_exchange_gemini,
        &mut limit_order_book,
    );

    let response_kraken_text = get_api_kraken(&client, user_agent);
    dump_kraken_response_text_to_file(&response_kraken_text);
    load_kraken_book(
        response_kraken_text,
        ticker_BTC_USD,
        source_exchange_kraken,
        &mut limit_order_book,
    );

    let target_volume = NotNan::new(10.0).unwrap();

    let total_cost_to_buy = cost_function_buy(&mut limit_order_book, ticker_BTC_USD, target_volume.clone());
    println!("Total cost to BUY {target_volume} BTC: ${total_cost_to_buy}");

    let total_profit_from_sell = profit_function_sell(&mut limit_order_book, ticker_BTC_USD, target_volume.clone());
    println!("Total profit from SELL {target_volume} BTC: ${total_profit_from_sell}");

    let total_cost_to_buy_by_source_exchange = 
        cost_function_buy_with_source_exchange(&mut limit_order_book, ticker_BTC_USD, target_volume.clone());
    println!("Total cost to BUY {target_volume} BTC by source exchange: {total_cost_to_buy_by_source_exchange:?}");

    let total_profit_from_sell_by_source_exchange = 
        profit_function_sell_with_source_exchange(&mut limit_order_book, ticker_BTC_USD, target_volume.clone());
    println!("Total profit from SELL {target_volume} BTC by source exchange: {total_profit_from_sell_by_source_exchange:?}");

    let total_volume_buy_by_source_exchange = 
        limit_order_book.total_volume_by_source_exchange(ticker_BTC_USD, &OrderSide::BUY);
    println!("Total volume BUY by source exchange: {total_volume_buy_by_source_exchange:?}");
    
    let total_volume_sell_by_source_exchange = 
        limit_order_book.total_volume_by_source_exchange(ticker_BTC_USD, &OrderSide::SELL);
    println!("Total volume SELL by source exchange: {total_volume_sell_by_source_exchange:?}");

    println!("Program ends");
}
