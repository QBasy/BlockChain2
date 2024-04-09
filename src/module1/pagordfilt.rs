use solana_client::{client::Client, rpc_client::RpcClient};

fn main() {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let program_data = rpc_client.get_program_data(
        &"YOUR_PROGRAM_ADDRESS_HERE".parse().unwrap(),
        PaginationParams {
            page: 1,
            page_size: 10,
            order: Order::Asc,
            filter: Some("FILTER_CRITERIA_HERE".to_string()),
        },
    );
    println!("{:?}", program_data);
}