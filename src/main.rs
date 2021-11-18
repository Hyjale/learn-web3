use std::env;
use std::str::FromStr;

use web3::types::{H160, U256};

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

    /* Account balances in Wei (1 ETH = 1^18 Wei)*/
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!(
            "ETH Balance of account {:?}: {}",
            account,
            balance.checked_div(U256::exp10(18)).unwrap()
        );
    }

    Ok(())
}
