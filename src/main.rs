use std::env;
use std::str::FromStr;

use web3::types::{Address, H160, U256};
use web3::contract::{Contract, Options};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let url = env::var("INFURA_RINKEBY").unwrap();
    let transport = web3::transports::WebSocket::new(&url).await?;
    let web3 = web3::Web3::new(transport);

    /* User Accounts */
    let address = H160::from_str(&env::var("USER_ADDRESS").unwrap()).unwrap();
    let mut accounts = web3.eth().accounts().await?;
    accounts.push(address);
    println!("Accounts: {:?}", accounts);

    /* Account balances in Wei (1 ETH = 1^18 Wei) */
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!(
            "ETH Balance of account {:?}: {}",
            account,
            balance.checked_div(U256::exp10(18)).unwrap()
        );
    }

    /* BAYC Contract */
    let bayc_address = Address::from_str("0x4A11d5b67F53162066DdE2f85b048Bb3C9dbA4F0").unwrap();
    let token_contract = Contract::from_json(web3.eth(), bayc_address, include_bytes!("erc20_abi.json")).unwrap();
    let token_name: String = token_contract
        .query("name", (), None, Options::default(), None)
        .await
        .unwrap();
    let total_token_supply: U256 = token_contract
        .query("totalSupply", (), None, Options::default(), None)
        .await
        .unwrap();

    println!("Token name: {}, total supply: {}", token_name, total_token_supply);

    Ok(())
}
