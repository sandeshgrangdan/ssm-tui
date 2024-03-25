// #[derive:(derive)]
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
// use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_ssm::{Client, Error};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("ap-southeast-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&config);

    println!("Getting parameters:");
    let resp = client.describe_parameters().max_results(50).send().await?;
    println!("Parameter store fetched:");

    let parameters = resp.parameters();

    for parameter in parameters {
        println!(" {:?}", parameter.name);
    }
    println!("Found {} value", parameters.len());

    println!();
    Ok(())
}
