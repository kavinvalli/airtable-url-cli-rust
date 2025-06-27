mod client;
mod config;
mod error;

use clap::{Parser, Subcommand};
use client::AirtableClient;
use config::AirtableConfig;
use owo_colors::OwoColorize;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        help = "Airtable secret token (uses env var AIRTABLE_SECRET_TOKEN if not provided)"
    )]
    secret_token: Option<String>,
    #[arg(
        short,
        long,
        help = "Airtable base (uses env var AIRTABLE_BASE if not provided)"
    )]
    base: Option<String>,
    #[arg(
        short,
        long,
        help = "Airtable table (uses env var AIRTABLE_TABLE if not provided)"
    )]
    table: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    GetAllRecords,
    GetRecordBySlug { slug: String },
    CreateRecord { slug: String, target: String },
    // UpdateRecord { slug: String, target: String },
    // DeleteRecord { slug: String },
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    let config = AirtableConfig::new(cli.secret_token, cli.base, cli.table);
    let client = AirtableClient::new(config);

    match &cli.command {
        Some(Commands::GetAllRecords) => {
            let records = client.get_all_records().await.unwrap_or_else(|e| {
                eprintln!("{}", e.to_string().red());
                Vec::new()
            });
            for record in records {
                println!("Record: {} -> {}", record.fields.slug, record.fields.target);
            }
        }
        Some(Commands::GetRecordBySlug { slug }) => match client.get_record_by_slug(slug).await {
            Ok(record) => println!("Target: {}", record.fields.target),
            Err(e) => eprintln!("{}", e.to_string().red()),
        },
        Some(Commands::CreateRecord { slug, target }) => {
            match client.create_record(slug, target).await {
                Ok(record) => println!("Record created: {:?}", record),
                Err(e) => eprintln!("{}", e.to_string().red()),
            }
        }
        // Some(Commands::UpdateRecord { slug, target }) => {
        //     let record = client.update_record(slug, target).await.unwrap();
        //     println!("Record updated: {:?}", record);
        // }
        // Some(Commands::DeleteRecord { slug }) => {
        //     client.delete_record(slug).await.unwrap();
        //     println!("Record deleted");
        // }
        None => {
            println!("No command provided");
        }
    }
}
