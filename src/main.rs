use anchor_client::{solana_sdk::signature::read_keypair_file, Client, Cluster};
use anyhow::Result;
use reqwest::Client as HttpClient;
use serde::Deserialize;
use std::path::PathBuf;

const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
const USDC_MINT: &str = "Es9vMFrzaCER51d1r93RWE6sYqQz1A1AVfWkrsCo8ix";

#[derive(Debug, Deserialize)]
struct MarketInfo {
    label: String,
}

#[derive(Debug, Deserialize)]
struct Route {
    out_amount: String,
    market_infos: Vec<MarketInfo>,
}

#[derive(Debug, Deserialize)]
struct QuoteResponse {
    data: Vec<Route>,
}

pub struct ArbitrageBot {
    _client: Client,
    http: HttpClient,
}

impl ArbitrageBot {
    pub fn new(keypair_path: impl Into<PathBuf>) -> Result<Self> {
        let payer = read_keypair_file(keypair_path.into())?;
        let client = Client::new(Cluster::Devnet, payer);
        Ok(Self {
            _client: client,
            http: HttpClient::new(),
        })
    }

    pub async fn check_arbitrage(&self) -> Result<()> {
        let url = "https://quote-api.jup.ag/v6/quote";
        let params = [
            ("inputMint", SOL_MINT),
            ("outputMint", USDC_MINT),
            ("amount", "1000000"), // 0.001 SOL
            ("slippageBps", "50"),
        ];

        let resp: QuoteResponse = self
            .http
            .get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        if resp.data.len() < 2 {
            println!("Not enough routes returned");
            return Ok(());
        }

        let buy = &resp.data[0];
        let sell = &resp.data[1];

        let buy_price: f64 = buy.out_amount.parse()?;
        let sell_price: f64 = sell.out_amount.parse()?;
        let profit = (sell_price - buy_price) / buy_price;

        if profit > 0.003 {
            let buy_dex = buy
                .market_infos
                .first()
                .map(|m| m.label.as_str())
                .unwrap_or("unknown");
            let sell_dex = sell
                .market_infos
                .first()
                .map(|m| m.label.as_str())
                .unwrap_or("unknown");
            println!(
                "Opportunity: buy on {} sell on {} profit {:.2}%",
                buy_dex,
                sell_dex,
                profit * 100.0
            );
        } else {
            println!("No profitable opportunity");
        }
        Ok(())
    }

    pub async fn simulate_swap(&self) -> Result<()> {
        // Actual transaction building and sending would go here. For now we just
        // print a message to keep the example simple.
        println!("Simulating swap (no transaction sent)");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let keypair_path = format!("{}/.config/solana/id.json", home);
    let bot = ArbitrageBot::new(keypair_path)?;
    bot.check_arbitrage().await?;
    bot.simulate_swap().await?;
    Ok(())
}
