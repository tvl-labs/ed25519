use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;

fn main() {
    // Generate a new keypair
    let mut csprng = OsRng {};
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);

    // The message to be signed
    let message: &[u8] = b"Hello, world!";

    // Sign the message
    let signature = signing_key.sign(message);

    // Print the signature
    println!("Signature: {:?}", signature.to_bytes());
}
