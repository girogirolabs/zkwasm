use crate::rpc::zk_wasm_client::ZkWasmClient;
use crate::rpc::SetupRequest;

pub mod rpc {
    tonic::include_proto!("rpc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ZkWasmClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SetupRequest {
        zkwasm_k: 18,
        wasm_image_id: "fibb".to_string(),
    });

    let response = client.setup(request).await?;

    println!("Received response {:?}", response);

    Ok(())
}
