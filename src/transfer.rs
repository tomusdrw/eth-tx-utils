use ethereum_types::{H160, U256};
use ethereum_transaction::{SignTransaction, Transaction};
use crate::utils::{debug, sign_transaction};

pub fn transfer(
    to: H160,
    nonce: U256,
    value: U256,
    gas_price: U256,
    chain_id: u64,
    key_path: &std::path::Path,
) -> Result<Vec<u8>, String> {
    let transaction = Transaction {
        from: Default::default(),
        to: Some(to),
        nonce,
        gas: U256::from_dec_str("21000").map_err(debug)?,
        gas_price,
        value,
        data: ethereum_transaction::Bytes(vec![]),
    };
    let to_sign = SignTransaction {
        transaction: std::borrow::Cow::Owned(transaction),
        chain_id,
    };
    let signed = sign_transaction(key_path, to_sign)?;
    let rlp = rlp::encode(&signed).to_vec();
    Ok(rlp)
}
