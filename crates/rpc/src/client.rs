use std::sync::Arc;
use std::sync::Mutex;

use crate::rpc::zk_wasm_client::ZkWasmClient;
use crate::rpc::{SetupRequest, ProveRequest, VerifyRequest};
// use delphinus_cli::args::ArgBuilder;

pub mod rpc {
    tonic::include_proto!("rpc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ZkWasmClient::connect("http://[::1]:50051").await?;

    let setup_request = tonic::Request::new(SetupRequest {
        zkwasm_k: 18,
        wasm_image_id: "fibb".to_string(),
    });
    let setup_response = client.setup(setup_request).await?;
    println!("Received setup response {:?}", setup_response);

    let public_inputs: Vec<u64> = vec![144];
    let private_inputs: Vec<u64> = vec![12];
    // let context_in = vec![];
    // let context_out = Arc::new(Mutex::new(vec![]));
    let prove_request = tonic::Request::new(ProveRequest {
        zkwasm_k: 18,
        wasm_image_id: "fibb".to_string(),
        public_inputs: public_inputs,
        private_inputs: private_inputs,
    });
    let prove_response = client.single_prove(prove_request).await?;
    println!("Received prove response {:?}", prove_response);

    let verify_request = tonic::Request::new(VerifyRequest {
        wasm_image_id: "fibb".to_string(),
    });
    let verify_response = client.single_verify(verify_request).await?;
    println!("Received prove response {:?}", verify_response);

    Ok(())
}
