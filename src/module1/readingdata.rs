use solana_client::{client::Client, rpc_client::RpcClient};

fn main() {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let account_data = rpc_client.get_account_data(&"YOUR_ACCOUNT_ADDRESS_HERE".parse().unwrap());
    println!("{:?}", account_data);
}