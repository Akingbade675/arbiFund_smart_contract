use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, U256},
};
use dotenv::dotenv;
use eyre::eyre;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

/// Your private key file path.
const PRIV_KEY_PATH: &str = "PRIV_KEY_PATH";

/// Stylus RPC endpoint url.
const RPC_URL: &str = "RPC_URL";

/// Deployed program address.
const STYLUS_CONTRACT_ADDRESS: &str = "STYLUS_CONTRACT_ADDRESS";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    let priv_key_path =
        std::env::var(PRIV_KEY_PATH).map_err(|_| eyre!("No {} env var set", PRIV_KEY_PATH))?;
    let rpc_url = std::env::var(RPC_URL).map_err(|_| eyre!("No {} env var set", RPC_URL))?;
    let contract_address = std::env::var(STYLUS_CONTRACT_ADDRESS)
        .map_err(|_| eyre!("No {} env var set", STYLUS_CONTRACT_ADDRESS))?;
    
    abigen!(
        CrowdFunding,
        r#"[
            function create_campaign(address owner, string memory title, string memory description, uint256 target, uint256 deadline, string memory image) external returns (uint256)
            function donate_to_campaign(uint256 campaign_id) external payable
            function get_campaigns() external view returns (address[] memory, string[] memory, string[] memory, uint256[] memory, uint256[] memory, string[] memory, address[][] memory, uint256[][] memory)
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = contract_address.parse()?;

    let privkey = read_secret_from_file(&priv_key_path)?;
    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let crowdfunding = CrowdFunding::new(address, client.clone());

    // Create a new campaign
    let owner = wallet.address();
    let title = "Test Campaign".to_string();
    let description = "This is a test campaign".to_string();
    let target = U256::from(1000000000000000000u64); // 1 ETH
    let deadline = U256::from(1680000000u64); // Set an appropriate deadline timestamp
    let image = "https://example.com/image.jpg".to_string();

    let tx = crowdfunding.create_campaign(owner, title, description, target, deadline, image);
    let receipt = tx.send().await?.await?;
    println!("Campaign created. Receipt: {:?}", receipt);

    // Get the campaign ID from the transaction logs (assuming it's the first topic)
    let campaign_id = receipt
        .logs
        .first()
        .and_then(|log| log.topics.get(1))
        .map(|topic| U256::from_big_endian(&topic.0))
        .ok_or_else(|| eyre!("Failed to get campaign ID from logs"))?;

    println!("Created campaign with ID: {}", campaign_id);

    // Donate to the campaign
    let donation_amount = U256::from(100000000000000000u64); // 0.1 ETH
    let tx = crowdfunding.donate_to_campaign(campaign_id);
    let receipt = tx.value(donation_amount).send().await?.await?;
    println!("Donation made. Receipt: {:?}", receipt);

    // Get all campaigns
    let campaigns = crowdfunding.get_campaigns().call().await?;
    println!("All campaigns: {:?}", campaigns);

    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    println!("Secret read from file", secret);
    Ok(secret.trim().to_string())
}