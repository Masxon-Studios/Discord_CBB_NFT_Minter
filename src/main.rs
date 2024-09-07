use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use std::env;
use web3::transports::Http;
use web3::types::{Address, TransactionParameters, U256, Bytes};
use web3::contract::Contract;
use web3::signing::SecretKey;
use std::str::FromStr;
use web3::ethabi::Token;
use rustc_hex::FromHexError;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![mintbadge()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("Bot is connected!");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(())
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn mintbadge(
    ctx: Context<'_>,
    #[description = "The username of the recipient"] username: String,
    #[description = "The wallet address of the recipient"] wallet_address: String,
    #[description = "The name of the badge"] badge_name: String,
) -> Result<(), Error> {
    // Construct the IPFS URL for the metadata
    let token_uri = "https://gateway.pinata.cloud/ipfs/QmVfV7TM8XZJJn8YRCf68JVaF4Hc1xCb1EDaz1WaAgdkdV".to_string();

    match mint_badge_command(username.clone(), wallet_address.clone(), token_uri).await {
        Ok(tx_hash) => {
            ctx.say(format!("Badge '{}' minted successfully for user {} (address: {}). Tx hash: {}", badge_name, username, wallet_address, tx_hash)).await?;
        }
        Err(err) => {
            ctx.say(format!("Error minting badge '{}' for user {} (address: {}): {}", badge_name, username, wallet_address, err)).await?;
        }
    }
    Ok(())
}

async fn mint_badge_command(username: String, wallet_address: String, token_uri: String) -> Result<String, String> {
    let klaytn_provider = "https://api.baobab.klaytn.net:8651";
    let transport = Http::new(klaytn_provider).map_err(|e| format!("Failed to connect to Klaytn: {}", e))?;
    let web3 = web3::Web3::new(transport);

    let default_account = env::var("KLAYTN_DEFAULT_ACCOUNT").expect("Expected a default account in the environment");
    let private_key = env::var("PRIVATE_KEY").expect("Expected a private key in the environment");
    let contract_address = env::var("CONTRACT_ADDRESS").expect("Expected a contract address in the environment");
    let contract_abi = env::var("CONTRACT_ABI").expect("Expected a contract ABI in the environment");

    let sender_address: Address = default_account.parse().map_err(|e: FromHexError| format!("Invalid sender address: {}", e))?;
    let contract_address: Address = contract_address.parse().map_err(|e: FromHexError| format!("Invalid contract address: {}", e))?;
    let recipient_address: Address = wallet_address.parse().map_err(|e: FromHexError| format!("Invalid wallet address: {}", e))?;
    let private_key = SecretKey::from_str(&private_key).map_err(|e| format!("Invalid private key: {}", e))?;
    let contract = Contract::from_json(web3.eth(), contract_address, contract_abi.as_bytes())
        .map_err(|e| format!("Failed to create contract instance: {}", e))?;

    let nonce = web3.eth().transaction_count(sender_address, None).await
        .map_err(|e| format!("Failed to get nonce: {}", e))?;

    println!("Preparing to mint badge for user: {}, address: {}", username, wallet_address);

    let tx_object = TransactionParameters {
        to: Some(contract_address),
        nonce: Some(nonce),
        gas: U256::from(500000),  // Increased gas limit
        gas_price: None,
        value: U256::zero(),
        data: Bytes::from(contract.abi().function("mintBadge").map_err(|e| format!("Failed to find mintBadge function: {}", e))?
            .encode_input(&[
                Token::Address(recipient_address),
                Token::String(token_uri),
            ]).map_err(|e| format!("Failed to encode function arguments: {}", e))?),
        ..Default::default()
    };

    let signed_tx = web3.accounts().sign_transaction(tx_object, &private_key).await
        .map_err(|e| format!("Failed to sign transaction: {}", e))?;
    
    println!("Sending transaction to mint badge for user: {}...", username);
    let result = web3.eth().send_raw_transaction(signed_tx.raw_transaction).await;

    match result {
        Ok(tx_hash) => {
            println!("Transaction sent successfully. Hash: {:?}", tx_hash);
            Ok(format!("{:?}", tx_hash))
        },
        Err(err) => {
            println!("Transaction failed. Error: {}", err);
            Err(format!("Transaction failed: {}", err))
        },
    }
}
