use anchor_client::Client;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, read_keypair_file},
    commitment_config::CommitmentConfig,
};
use clap::Parser;
use anyhow::Result;
use std::rc::Rc;
use dotenv::dotenv;
use std::env;

/// Boom Token Minter CLI
#[derive(Parser)]
#[command(name = "Boom Token Minter", version)]
struct Opts {
    /// Path to Solana keypair JSON file
    #[arg(short, long)]
    keypair: String,

    /// Anchor program ID (Pubkey)
    #[arg(short, long)]
    program_id: String,

    /// RPC endpoint
    #[arg(short, long, default_value = "https://api.mainnet-beta.solana.com")]
    rpc: String,
}

fn main() -> Result<()> {
    dotenv().ok();
    let opts = Opts::parse();

    let payer = read_keypair_file(&opts.keypair)?;
    let program_id: Pubkey = opts.program_id.parse()?;
    let cluster = opts.rpc.clone();

    let client = Client::new_with_options(
        cluster,
        Rc::new(payer),
        CommitmentConfig::processed(),
    );

    let program = client.program(program_id);
    println!("✅ Connected to program: {}", program.id());

    // Insert minting logic here
    println!("🚀 Token minting ready...");

    Ok(())
}
