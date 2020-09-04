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

pub fn get_nonce(
    rpc: &Option<String>,
    nonce: &Option<U256>,
    key_path: &std::path::Path,
) -> Result<U256, String> {
    match (rpc.as_ref(), nonce.as_ref()) {
        (Some(url), None) => {
            let address = crate::utils::read_keyfile_address(key_path)?;
            println!("Retrieving nonce for {:?}", address);
            crate::web3::send(&url, |rt, web3| rt.block_on(
                web3.eth().transaction_count(address, None)
            )).map_err(debug)
        },
        (_, Some(nonce)) => Ok(nonce.clone()),
        (None, None) => Err("No RPC nor Nonce provided.".into()),
    }
}
