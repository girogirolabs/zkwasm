use std::env;
use std::fs;
use std::path::PathBuf;

// Use current directory as the default asset root
fn get_fallback_asset_root() -> PathBuf {
    let mut current_dir = env::current_dir().unwrap_or_default();
    current_dir.push("test-assets");
    current_dir
}

fn get_asset_root() -> PathBuf {
    match env::var("ZKWASM_ASSET_ROOT") {
        Ok(val) => PathBuf::from(val),
        Err(_) => get_fallback_asset_root(),
    }
}

pub fn get_wasm_image_path(binary_id: &str) -> PathBuf {
    let mut wasm_image_path = get_asset_root();
    wasm_image_path.extend(&["image", &format!("{}.wasm", binary_id)]);
    wasm_image_path
}

pub fn get_output_dir(circuit_id: &str) -> PathBuf {
    let mut output_dir = get_asset_root();
    output_dir.extend(&["output", circuit_id]);
    output_dir
}

pub fn get_and_create_output_dir(circuit_id: &str) -> PathBuf {
    let output_dir = get_output_dir(circuit_id);
    fs::create_dir_all(output_dir.clone()).expect(&format!("Failed to create output direcotry at {}", output_dir.display()));
    output_dir
}

pub fn get_params_dir(circuit_id: &str) -> PathBuf {
    let mut params_dir = get_asset_root();
    params_dir.extend(&["params", circuit_id]);
    params_dir
}

pub fn get_and_create_params_dir(circuit_id: &str) -> PathBuf {
    let params_dir = get_params_dir(circuit_id);
    fs::create_dir_all(params_dir.clone()).expect(&format!("Failed to create params direcotry at {}", params_dir.display()));
    params_dir
}
