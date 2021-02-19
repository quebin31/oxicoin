use std::env;

use anyhow::{anyhow, Result};
use iotacoin::secp256k1::crypto::PrivateKey;
use iotacoin::utils::hash256;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 2 {
        println!("Usage: program <secret>");
        return Err(anyhow!("Invalid number of args"));
    }

    let secret_digest = hash256(&args[1]);
    let private_key = PrivateKey::from_bytes_be(secret_digest);
    let public_key = private_key.public_key();

    println!("Main address: {:?}", public_key.create_address(true, false));
    println!("Test address: {:?}", public_key.create_address(true, true));
    println!("Main WIF: {:?}", private_key.create_wif(true, false));
    println!("Test WIF: {:?}", private_key.create_wif(true, true));

    Ok(())
}
