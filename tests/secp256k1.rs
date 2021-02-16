use anyhow::Result;
use hex_literal::hex;
use iotacoin::biguint;
use iotacoin::secp256k1::crypto::{PrivateKey, PublicKey};
use iotacoin::secp256k1::curve::Point;
use iotacoin::secp256k1::signature::Signature;
use num_bigint::BigUint;

#[test]
fn signature_must_be_valid() -> Result<()> {
    let digest = hex!("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423");

    let r = biguint!("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6");
    let s = biguint!("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
    let signature = Signature::new(r, s);

    let x = biguint!("04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574");
    let y = biguint!("82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4");
    let pub_key = PublicKey::new(x, y).unwrap();

    assert!(signature.is_valid(&digest, &pub_key).unwrap());
    Ok(())
}

#[test]
fn create_and_validate_signature() -> Result<()> {
    let privkey = PrivateKey::new(BigUint::from(12345usize));
    let digest = hex!("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423");

    let signature = privkey.create_signature(&digest)?;

    insta::assert_debug_snapshot!(signature); // signature shouldn't change
    assert!(privkey
        .public_key()
        .valid_signature(&digest, &signature)
        .unwrap());

    Ok(())
}

#[test]
fn uncompressed_sec_format() {
    fn test_case(secret: usize, expected: &[u8]) {
        let private_key = PrivateKey::new(secret);
        let public_key = private_key.public_key();
        let serialized = public_key.serialize(false).unwrap();

        assert_eq!(serialized, expected);
        let deserialized: PublicKey = Point::deserialize(&serialized).unwrap().into();
        assert_eq!(&deserialized, public_key);
    }

    test_case(
        5000,
        &hex!(
            "04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4
            f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10"
        ),
    );

    test_case(
        33466154331649568,
        &hex!(
            "04027f3da1918455e03c46f659266a1bb5204e959db7364d2f473bdf8f0a13cc9dff87647fd023
            c13b4a4994f17691895806e1b40b57f4fd22581a4f46851f3b06"
        ),
    );

    test_case(
        3917405024756549,
        &hex!(
            "04d90cd625ee87dd38656dd95cf79f65f60f7273b67d3096e68bd81e4f5342691f842efa762fd5
            9961d0e99803c61edba8b3e3f7dc3a341836f97733aebf987121"
        ),
    );
}

#[test]
fn compressed_sec_serialization() {
    fn test_case(secret: usize, expected: &[u8]) {
        let private_key = PrivateKey::new(secret);
        let public_key = private_key.public_key();
        let serialized = public_key.serialize(true).unwrap();

        assert_eq!(serialized, expected);
        let deserialized: PublicKey = Point::deserialize(&serialized).unwrap().into();
        assert_eq!(&deserialized, public_key);
    }

    test_case(
        5001,
        &hex!("0357a4f368868a8a6d572991e484e664810ff14c05c0fa023275251151fe0e53d1"),
    );

    test_case(
        33549155665686099,
        &hex!("02933ec2d2b111b92737ec12f1c5d20f3233a0ad21cd8b36d0bca7a0cfa5cb8701"),
    );

    test_case(
        3917405025026849,
        &hex!("0296be5b1292f6c856b3c5654e886fc13511462059089cdf9c479623bfcbe77690"),
    );
}

#[test]
fn address_creation() {
    fn test_case(secret: usize, compressed: bool, testnet: bool, expected: &str) {
        let private_key = PrivateKey::new(secret);
        let public_key = private_key.public_key();
        let address = public_key.create_address(compressed, testnet).unwrap();

        assert_eq!(expected, address);
    }

    test_case(5002, false, true, "mmTPbXQFxboEtNRkwfh6K51jvdtHLxGeMA");
    test_case(
        33632321603200000,
        true,
        true,
        "mopVkxp8UhXqRYbCYJsbeE1h1fiF64jcoH",
    );
    test_case(
        320257972354799,
        true,
        false,
        "1F1Pn2y6pDb68E5nYJJeba4TLg2U7B6KF1",
    );
}
