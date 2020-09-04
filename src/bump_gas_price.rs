use ethereum_transaction::{SignedTransaction, SignTransaction};
use ethereum_types::U256;
use ethsign::Signature;
use crate::utils::{debug, sign_transaction};
use rlp::Decodable;
use rustc_hex::ToHex;

pub fn bump_gas_price(gas_price: U256, rlp: &[u8], key_path: &std::path::Path) -> Result<Vec<u8>, String> {
    let mut tx = SignedTransaction::decode(&rlp::Rlp::new(rlp)).map_err(debug)?;
    // check correctness of the transaction
    let signature = Signature {
        v: tx.standard_v(),
        r: tx.r.into(),
        s: tx.s.into(),
    };
    let pubkey = signature.recover(&tx.bare_hash()).map_err(debug)?;
    println!("Recovered: {:?}", pubkey.address().to_hex::<String>());
    // alter the gas price
    println!(
        "GAS PRICE:\n  Current: {gp:x} ({gp})\n  New: {new:x} ({new})",
        gp = tx.transaction.gas_price,
        new = gas_price
    );
    tx.transaction.to_mut().gas_price = gas_price;

    // get the secret to sign transaction
    let chain_id = tx.chain_id().unwrap_or_default();
    let signed = sign_transaction(key_path, SignTransaction {
        transaction: tx.transaction,
        chain_id
    })?;
    let rlp = rlp::encode(&signed).to_vec();
    Ok(rlp)
}

