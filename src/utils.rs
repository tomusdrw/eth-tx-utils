use ethsign::{keyfile::KeyFile, Protected};

pub fn open_keyfile(key_path: &std::path::Path) -> Result<ethsign::SecretKey, String> {
    let keyfile = std::fs::File::open(key_path).map_err(debug)?;
    let key: KeyFile = serde_json::from_reader(keyfile).map_err(debug)?;
    let password: Protected = rpassword::prompt_password_stdout(
        &format!("Password for {:?}: ", key.address)
    ).map_err(debug)?.into();
    key.to_secret_key(&password).map_err(debug)
}

pub fn debug<T: std::fmt::Debug>(t: T) -> String {
    format!("{:?}", t)
}
