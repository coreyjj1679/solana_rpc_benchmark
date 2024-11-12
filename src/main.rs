use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

#[derive(Deserialize)]
struct RpcEndpoint {
    endpoint: String,
    name: String,
}

fn load_endpoints_from_file(file_path: &str) -> Vec<RpcEndpoint> {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let endpoints: Vec<RpcEndpoint> =
        serde_json::from_reader(reader).expect("Unable to parse JSON");
    endpoints
}

async fn benchmark_endpoint(client: &RpcClient, public_key: &solana_sdk::pubkey::Pubkey) -> u128 {
    let start = Instant::now();
    let _ = client.get_balance(public_key);
    start.elapsed().as_millis()
}

#[tokio::main]
async fn main() {
    let endpoints = load_endpoints_from_file("src/endpoints.json");

    for endpoint in endpoints {
        let client = RpcClient::new(endpoint.endpoint.clone());
        let keypair = Keypair::new();
        let public_key = keypair.pubkey();
        let mut total_duration = 0u128;
        let tests = 100;

        for _ in 0..tests {
            total_duration += benchmark_endpoint(&client, &public_key).await;
        }

        let average_duration = total_duration / tests as u128;

        println!(
            "RPC Endpoint: {} | Name: {} | Average Duration: {} ms",
            endpoint.endpoint, endpoint.name, average_duration
        );
    }
}
