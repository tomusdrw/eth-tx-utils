extern crate common_types;
extern crate ethkey;
extern crate ethstore;
extern crate rlp;
extern crate rpassword;
extern crate rustc_hex;
extern crate serde_json;

use common_types::transaction::{UnverifiedTransaction, SignedTransaction};
use rlp::Decodable;
use rustc_hex::{FromHex, ToHex};

fn main() {
    let mut it = ::std::env::args();
    // skip binary name
    it.next();
    let gas_price = it.next();
    let rlp = it.next();
    let key_path = it.next();
    let result = match (gas_price, rlp, key_path) {
        (Some(gas_price), Some(rlp), Some(key_path)) => {
            bump_gas_price(&gas_price, &rlp, key_path.as_ref())
        },
        (None, _, _) => Err("Please provide a new gas price as the first argument.".into()),
        (_, None, _) => Err("Please provide a raw transaction (RLP) as the second argument.".into()),
        (_, _, None) => Err("Please provide key path as the third argument.".into()),
    };

    match result {
        Ok(o) => println!("{}", o),
        Err(e) => println!(r#"Error:
{:?}

Usage: <bin> <NewGasPrice> <RLP> <KeyPath>
"#, e),
    }
}

fn bump_gas_price(gas_price: &str, rlp: &str, key_path: &::std::path::Path) -> Result<String, String> {
    let rlp: Vec<_> = rlp.from_hex().map_err(debug)?;
    let gas_price = gas_price.parse().map_err(debug)?;
    let tx = UnverifiedTransaction::decode(&rlp::Rlp::new(&rlp)).map_err(debug)?;
    // check correctness of the transaction
    let (tx, address, _) = SignedTransaction::new(tx).map_err(debug)?.deconstruct();
    // alter gas price
    let chain_id = tx.chain_id();
    let mut unsigned = tx.as_unsigned().clone();

    // alter the gas price
    println!(
        "GAS PRICE:\n  Current: {gp:x} ({gp})\n  New: {new:x} ({new})",
        gp = unsigned.gas_price,
        new = gas_price
    );
    unsigned.gas_price = gas_price;

    // get the secret to sign transaction
    let signature = sign(key_path, &unsigned.hash(chain_id))?;
    let signed = unsigned.with_signature(signature, chain_id);

    let rlp = rlp::encode(&signed).to_vec();
    let s: String = rlp.to_hex();
    Ok(format!("RLP: {}", s))
}

fn sign(key_path: &::std::path::Path, message: &ethkey::Message) -> Result<ethkey::Signature, String> {
    // THE API IS FUCKING UGLY! Why can't I just create SafeAccount by deserializing the fucking JSON?
    let disk = ethstore::accounts_dir::DiskKeyFileManager::default();
    let account = ethstore::accounts_dir::KeyFileManager::read(
        &disk,
        key_path.to_str().map(str::to_owned),
        ::std::fs::File::open(key_path).map_err(debug)?,
    ).map_err(debug)?;

    let password = rpassword::prompt_password_stdout(&format!("Password for {}: ", account.address)).map_err(debug)?;
    account.sign(&password.into(), message).map_err(debug)
}

fn debug<T: ::std::fmt::Debug>(t: T) -> String {
    format!("{:?}", t)
}
