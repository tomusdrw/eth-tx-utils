use ethereum_transaction::SignedTransaction;
use ethsign::Signature;
use rlp::Decodable;
use rustc_hex::{FromHex, ToHex};
use crate::utils::{debug, open_keyfile};

pub fn bump_gas_price(gas_price: &str, rlp: &str, key_path: &std::path::Path) -> Result<String, String> {
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

