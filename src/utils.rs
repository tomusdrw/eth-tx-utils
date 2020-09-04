use ethereum_types::H160;
use ethereum_transaction::{SignTransaction, SignedTransaction};
use ethsign::{keyfile::KeyFile, Protected};
use rustc_hex::ToHex;

pub fn read_keyfile_address(key_path: &std::path::Path) -> Result<H160, String> {
    let keyfile = std::fs::File::open(key_path).map_err(debug)?;
    let key: KeyFile = serde_json::from_reader(keyfile).map_err(debug)?;
    let addr = key.address.as_ref().ok_or_else(|| "No address in JSON.".to_owned())?;
    Ok(H160::from_slice(&addr.0))
}

pub fn open_keyfile(key_path: &std::path::Path) -> Result<ethsign::SecretKey, String> {
    let keyfile = std::fs::File::open(key_path).map_err(debug)?;
    let key: KeyFile = serde_json::from_reader(keyfile).map_err(debug)?;
    let address = key.address.as_ref().map(|x| x.0.to_hex::<String>());
    let password: Protected = rpassword::prompt_password_stdout(
        &format!("Password for {:?}: ", address)
    ).map_err(debug)?.into();
    key.to_secret_key(&password).map_err(debug)
}

pub fn debug<T: std::fmt::Debug>(t: T) -> String {
    format!("{:?}", t)
}

pub fn sign_transaction<'a>(
    key_file: &std::path::Path, 
    tx: SignTransaction<'a>
) -> Result<SignedTransaction<'a>, String> {
    println!("Signing transaction: {:?}", tx);
    let secret_key = open_keyfile(key_file)?;
    let signature = secret_key.sign(&tx.hash()).map_err(debug)?;
    Ok(SignedTransaction::new(
        tx.transaction,
        tx.chain_id,
        signature.v,
        signature.r,
        signature.s,
    ))
}
