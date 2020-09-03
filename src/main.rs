use ethereum_transaction::SignedTransaction;
use ethsign::{Signature, keyfile::KeyFile, Protected};
use rlp::Decodable;
use rustc_hex::{FromHex, ToHex};
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "eth-tx-util", about = "Ethereum transaction utilities")]
struct BumpGasPrice {
    /// New gas price of the transaction.
    gas_price: String,
    /// Hex-encoded RLP of the transaction.
    rlp: String,
    /// Path to a JSON key file.
    key_path: std::path::PathBuf,
}

fn main() -> Result<(), String> {
    let opt = BumpGasPrice::from_args();
    let result = bump_gas_price(&opt.gas_price, &opt.rlp, &opt.key_path.as_ref())?;
    println!("{}", result);
    Ok(())
}

fn bump_gas_price(gas_price: &str, rlp: &str, key_path: &std::path::Path) -> Result<String, String> {
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
    let secret_key = open_keyfile(key_path)?;
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

fn open_keyfile(key_path: &std::path::Path) -> Result<ethsign::SecretKey, String> {
    let keyfile = std::fs::File::open(key_path).map_err(debug)?;
    let key: KeyFile = serde_json::from_reader(keyfile).map_err(debug)?;
    let password: Protected = rpassword::prompt_password_stdout(
        &format!("Password for {:?}: ", key.address)
    ).map_err(debug)?.into();
    key.to_secret_key(&password).map_err(debug)
}

fn debug<T: std::fmt::Debug>(t: T) -> String {
    format!("{:?}", t)
}
