use clap::Parser;
use reqwest;
use tokio;
use std::collections::HashMap;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The command to look for
    pair: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let pair = args.pair;
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", pair);
    let res = reqwest::get(&url)
        .await?
        .json::<HashMap<String, HashMap<String, f64>>>()
        .await?;
    println!("{} price is: {}", pair, res.get(&pair).unwrap().get("usd").unwrap());

    Ok(())
}
