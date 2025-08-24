use snarkvm_console::network::MainnetV0;
use wasm_bindgen::prelude::*;

type AddressMainnet = snarkvm_console::account::Address<MainnetV0>;

/// Verify a mainnet signature with a mainnet address and message
#[wasm_bindgen]
pub fn verify_signature(address: &str, message: &str, signature: &str) -> Result<bool, String> {
    let address = address
        .parse::<AddressMainnet>()
        .map_err(|e| e.to_string())?;
    let signature = signature
        .parse::<snarkvm_console::account::Signature<MainnetV0>>()
        .map_err(|e| e.to_string())?;
    Ok(signature.verify_bytes(&address, message.as_bytes()))
}

/// Verify a mainnet address
#[wasm_bindgen]
pub fn verify_address(address: &str) -> bool {
    address.parse::<AddressMainnet>().is_ok()
}
