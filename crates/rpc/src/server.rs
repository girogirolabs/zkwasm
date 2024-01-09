use std::fs;
use std::sync::Arc;
use std::sync::Mutex;

use tonic::{transport::Server, Request, Response, Status};

use crate::rpc::zk_wasm_server::{ZkWasm, ZkWasmServer};
use crate::rpc::{SetupRequest, SetupReply};
use crate::rpc::{ProveRequest, ProveReply};
use crate::rpc::{VerifyRequest, VerifyReply};

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

    async fn single_prove(
        &self,
        request: Request<ProveRequest>,
    ) -> Result<Response<ProveReply>, Status> {
        println!("Received prove request from {:?}", request.remote_addr());
        let prove_request = request.into_inner();
        let zkwasm_k = prove_request.zkwasm_k;
        let wasm_image_id = prove_request.wasm_image_id;
        let wasm_image_path = asset::get_wasm_image_path(&wasm_image_id);
        let wasm_image_bin: Vec<u8> = fs::read(&wasm_image_path).unwrap();
        let public_inputs: Vec<u64> = prove_request.public_inputs.to_vec();
        let private_inputs: Vec<u64> = prove_request.private_inputs.to_vec();
        let context_in = vec![];
        let context_out = Arc::new(Mutex::new(vec![]));
        let prove_res = exec::exec_create_proof::<ExecutionArg, DefaultHostEnvBuilder>(
            "zkwasm-rpc-server",
            zkwasm_k,
            wasm_image_bin,
            vec![],
            &asset::get_and_create_output_dir(&wasm_image_id),
            &asset::get_and_create_params_dir(&wasm_image_id),
            // create a ExecutionArg in server
            ExecutionArg {
                public_inputs,
                private_inputs,
                context_inputs: context_in,
                context_outputs: context_out.clone(),
            },
        );

        let reply = rpc::ProveReply {
            success: (match prove_res {
                Ok(_) => true,
                Err(_) => false,
            }),
            message: "Finished prove".to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn single_verify(
        &self,
        request: Request<VerifyRequest>,
    ) -> Result<Response<VerifyReply>, Status> {
        println!("Received verify request from {:?}", request.remote_addr());
        let verify_request = request.into_inner();
        let wasm_image_id = verify_request.wasm_image_id;
        let verify_res = exec::exec_verify_proof(
            "zkwasm-rpc-server",
            &asset::get_and_create_output_dir(&wasm_image_id),
            &asset::get_and_create_params_dir(&wasm_image_id),
        );

        let reply = rpc::VerifyReply {
            success: (match verify_res {
                Ok(_) => true,
                Err(_) => false,
            }),
            message: "Finished verify".to_string(),
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
