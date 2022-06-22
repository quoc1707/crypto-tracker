extern crate serde;
use serde::{Deserialize, Serialize};

extern crate reqwest;
use reqwest::Client;

extern crate clap;
use clap::Parser;

use std::error::Error;
use std::string::String;
use std::thread::sleep;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinbasePrice {
    pub data: CoinPrice,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinPrice {
    pub base: String,
    pub currency: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinbaseTime {
    pub data: CoinTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinTime {
    pub iso: String,
    pub epoch: i64,
}

#[derive(Parser, Debug)]

struct Cli {
    #[clap(short, long, default_value = "BTC")]
    first: String,
    #[clap(short, long, default_value = "ETH")]
    second: String,
    #[clap(short, long, default_value = "USDT")]
    third: String,
    #[clap(short, long, default_value = "USD")]
    rates: String,
    #[clap(short, long, default_value = "1000")]
    interval: u64,
    #[clap(short, long, default_value = "100")]
    frequency: i32,
}

pub fn crypto_publisher() {
    let args = Cli::parse();
    let mut count = 0i32;

    loop {
        if count == args.frequency {
            break;
        }

        let bitcoin = &args.first;
        let ethereum = &args.second;
        let tether = &args.third;
        let rates = &args.rates;

        let quote_time = get_coin_time();
        let quote_time2 = get_coin_time();
        let quote_time3 = get_coin_time();

        let spot_price = get_coin_price("spot".to_string(), bitcoin.to_string(), rates.to_string());
        let buy_price = get_coin_price("buy".to_string(), bitcoin.to_string(), rates.to_string());
        let sell_price = get_coin_price("sell".to_string(), bitcoin.to_string(), rates.to_string());

        let spot_price2 =
            get_coin_price("spot".to_string(), ethereum.to_string(), rates.to_string());
        let buy_price2 = get_coin_price("buy".to_string(), ethereum.to_string(), rates.to_string());
        let sell_price2 =
            get_coin_price("sell".to_string(), ethereum.to_string(), rates.to_string());

        let spot_price3 = get_coin_price("spot".to_string(), tether.to_string(), rates.to_string());
        let buy_price3 = get_coin_price("buy".to_string(), tether.to_string(), rates.to_string());
        let sell_price3 = get_coin_price("sell".to_string(), tether.to_string(), rates.to_string());

        let buy_price = buy_price.as_ref();
        let sell_price = sell_price.as_ref();

        let buy_price2 = buy_price2.as_ref();
        let sell_price2 = sell_price2.as_ref();

        let buy_price3 = buy_price3.as_ref();
        let sell_price3 = sell_price3.as_ref();

        let spread_price: f32 = (buy_price.unwrap().parse::<f32>().unwrap())
            - (&sell_price.unwrap().parse::<f32>().unwrap());
        let spread_price2: f32 = (buy_price2.unwrap().parse::<f32>().unwrap())
            - (&sell_price2.unwrap().parse::<f32>().unwrap());
        let spread_price3: f32 = (buy_price3.unwrap().parse::<f32>().unwrap())
            - (&sell_price3.unwrap().parse::<f32>().unwrap());

        println!(
            "{}: {}-{} SPOT Price: {} | BUY Price: {} | SELL Price: {} | Price Spread: {}",
            quote_time.unwrap(),
            bitcoin.to_string(),
            rates.to_string(),
            spot_price.unwrap(),
            buy_price.unwrap(),
            sell_price.unwrap(),
            spread_price.to_string()
        );
        println!(
            "{}: {}-{} SPOT Price: {} | BUY Price: {} | SELL Price: {} | Price Spread: {}",
            quote_time2.unwrap(),
            ethereum.to_string(),
            rates.to_string(),
            spot_price2.unwrap(),
            buy_price2.unwrap(),
            sell_price2.unwrap(),
            spread_price2.to_string()
        );
        println!(
            "{}: {}-{} SPOT Price: {} | BUY Price: {} | SELL Price: {} | Price Spread: {}",
            quote_time3.unwrap(),
            tether.to_string(),
            rates.to_string(),
            spot_price3.unwrap(),
            buy_price3.unwrap(),
            sell_price3.unwrap(),
            spread_price3.to_string()
        );

        sleep(Duration::from_millis(args.interval));

        count += 1;
    }
}

#[tokio::main]
async fn get_coin_price(
    request_type: String,
    request_currency: String,
    request_rates: String,
) -> Result<String, Box<dyn Error>> {
    let request_url = format!(
        "https://api.coinbase.com/v2/prices/{}-{}/{}",
        request_currency, request_rates, request_type
    );
    let client = Client::new();
    let resp_price = client
        .get(&request_url)
        .send()
        .await?
        .json::<CoinbasePrice>()
        .await?;

    Ok(resp_price.data.amount)
}

#[tokio::main]
async fn get_coin_time() -> Result<String, Box<dyn Error>> {
    let request_url = format!("https://api.coinbase.com/v2/time");
    let client = Client::new();
    let resp_time = client
        .get(&request_url)
        .send()
        .await?
        .json::<CoinbaseTime>()
        .await?;

    Ok(resp_time.data.iso)
}
