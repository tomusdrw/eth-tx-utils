type Web3 = web3::Web3<web3::transports::Http>;

pub fn send<F, R>(url: &str, f: F) -> web3::Result<R>
where
    F: FnOnce(&mut tokio::runtime::Runtime, Web3) -> web3::Result<R>,
{
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let transport = web3::transports::Http::new(url)?;
    let web3 = web3::Web3::new(transport);
    f(&mut rt, web3)
}
