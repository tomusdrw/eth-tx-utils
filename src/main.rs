use ethereum_transaction::SignedTransaction;
use ethsign::{Signature, keyfile::KeyFile, Protected};
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
    let mut tx = SignedTransaction::decode(&rlp::Rlp::new(&rlp)).map_err(debug)?;
    // check correctness of the transaction
    let signature = Signature {
        v: tx.standard_v(),
        r: tx.r.into(),
        s: tx.s.into(),
    };
    let pubkey = signature.recover(&tx.bare_hash()).map_err(debug)?;
    println!("Recovered: {:?}", pubkey.address());
    // alter the gas price
    println!(
        "GAS PRICE:\n  Current: {gp:x} ({gp})\n  New: {new:x} ({new})",
        gp = tx.transaction.gas_price,
        new = gas_price
    );
    tx.transaction.to_mut().gas_price = gas_price;

    // get the secret to sign transaction
    let keyfile = std::fs::File::open(key_path).map_err(debug)?;
    let key: KeyFile = serde_json::from_reader(keyfile).map_err(debug)?;
    let password: Protected = rpassword::prompt_password_stdout(
        &format!("Password for {:?}: ", key.address)
    ).map_err(debug)?.into();
    let secret_key = key.to_secret_key(&password).map_err(debug)?;
    let signature = secret_key.sign(&tx.bare_hash()).map_err(debug)?;

    let chain_id = tx.chain_id().unwrap_or_default();
    let transaction = SignedTransaction::new(
        tx.transaction,
        chain_id,
        signature.v,
        signature.r,
        signature.s,
    );
    let rlp = rlp::encode(&transaction).to_vec();
    let s: String = rlp.to_hex();
    Ok(format!("RLP: {}", s))
}

fn debug<T: ::std::fmt::Debug>(t: T) -> String {
    format!("{:?}", t)
}
