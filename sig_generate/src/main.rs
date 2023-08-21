mod bindings {
    pub mod src {
        pub mod ed_25519;
    }
}

// use bindings::src::ed_25519::*;
use ethers::{
    contract::abigen,
    core::{
        types::{Address, Bytes as EthByte, U256},
        utils::Anvil,
    },
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};
use std::{sync::Arc, time::Duration};
// use ethers::{prelude::*, providers::Provider, types::Address};
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;

abigen!(Ed25519, "/home/ubuntu/ed25519/out/ed25519.sol/Ed25519.json");

fn generate_key_pair() -> (SigningKey, VerifyingKey) {
    let mut csprng = OsRng {};
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    return (signing_key, verifying_key);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sk, vk) = generate_key_pair();

    let anvil = Anvil::new().spawn();
    let provider = Provider::<Http>::try_from(anvil.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64));
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let contract = Ed25519::new(Address::zero(), client);
    // The message to be signed
    let message: EthByte = EthByte::from(b"Hello, world!".to_vec());

    // Sign the message
    let signature = sk.sign(&message);
    let signature_bytes = EthByte::from(signature.to_bytes().to_vec());

    //  We generate the CompressedEdwardsY , we need to split into into the (x,y)
    // the first 255 buts represent y, and the high bit of the 32nd byte gives the sign of x
    // The only method that exposes it , is . Not sure if we need to use the compressedY or the Edwards one.

    // let vk_compressed = vk.to_bytes();
    // let vk_decompressed = VerifyingKey::from_bytes(&vk_compressed).unwrap();
    // vk_decompressed.point;
    // let mut first_255_bits = vk_compressed[0..32].to_vec();
    // let last_byte = first_255_bits[31];
    // first_255_bits[31] = last_byte & 0b0111_1111;

    // TODO: Fix this . currently overflows because I am slicing it wrong and probably using the wrong point.
    let vk_bytes = vk.to_bytes();
    let vk_part1 = U256::from(&vk_bytes[0..32]);
    let vk_part2 = U256::from(&vk_bytes[32..64]);
    let vk_u256 = [vk_part1, vk_part2];

    let result = contract
        .verify_signature(message, signature_bytes, vk_u256)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e) as Box<dyn std::error::Error>),
    }
}
