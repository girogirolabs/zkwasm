use std::fs;

use tonic::{transport::Server, Request, Response, Status};

use crate::rpc::zk_wasm_server::{ZkWasm, ZkWasmServer};
use crate::rpc::{SetupRequest, SetupReply};

use delphinus_zkwasm::runtime::host::default_env::DefaultHostEnvBuilder;
use delphinus_zkwasm::runtime::host::default_env::ExecutionArg;
use delphinus_cli::exec;

pub mod rpc {
    tonic::include_proto!("rpc");
}
pub mod asset;

#[derive(Debug, Default)]
pub struct ZkWasmRpcServer {}

#[tonic::async_trait]
impl ZkWasm for ZkWasmRpcServer {
    async fn setup(
        &self,
        request: Request<SetupRequest>,
    ) -> Result<Response<SetupReply>, Status> {
        println!("Received setup request from {:?}", request.remote_addr());
        let setup_request = request.into_inner();
        let zkwasm_k = setup_request.zkwasm_k;
        let wasm_image_id = setup_request.wasm_image_id;
        let wasm_image_path = asset::get_wasm_image_path(&wasm_image_id);
        let wasm_image_bin: Vec<u8> = fs::read(&wasm_image_path).unwrap();
        let setup_res = exec::exec_setup::<ExecutionArg, DefaultHostEnvBuilder>(
            zkwasm_k,
            22,
            "zkwasm-rpc-server",
            wasm_image_bin,
            vec![],
            &asset::get_and_create_output_dir(&wasm_image_id),
            &asset::get_and_create_params_dir(&wasm_image_id),
        );

        let reply = rpc::SetupReply {
            success: (match setup_res {
                Ok(_) => true,
                Err(_) => false,
            }),
            message: "Finished setup".to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let server = ZkWasmRpcServer::default();

    println!("ZkWasmRpcServer listening on {}", addr);

    Server::builder()
        .add_service(ZkWasmServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
