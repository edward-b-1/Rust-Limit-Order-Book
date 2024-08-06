

use reqwest;
use std::io::Write;


fn main() {
    println!("Program start");

    let client = reqwest::blocking::Client::new();

    // The behaviour I want here is for the program to panic if this call fails, hence `unwrap`
    let response = client
        .get("https://api.exchange.coinbase.com/products")
        .header("Content-Type", "application/json")
        .send()
        .expect("client failed to send request");
    println!("{:?}", response);
    println!("Response Status Code: {}", response.status());
    
    let response_text = response.text().expect("failed to convert response to text");
    
    let filename = "coinbase_products.json";
    let mut file = std::fs::File::create(filename).unwrap_or_else(|error| panic!("failed to create file {filename}: {error}"));
    file.write(response_text.as_bytes()).expect("failed to write data to file");

    println!("Program ends")
}
