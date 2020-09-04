use structopt::StructOpt;
use ethereum_types::{U256, H160};
use rustc_hex::{FromHex, ToHex};

mod bump_gas_price;
mod transfer;
mod utils;
mod web3;

#[derive(Debug, StructOpt)]
#[structopt(name = "eth-tx-util", about = "Ethereum transaction utilities")]
enum Opt {
    BumpGasPrice(BumpGasPrice),
    Transfer(Transfer),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "bump-gas-price", about = "Get a signed transaction with increased gas price given a RLP of signed transaction")]
struct BumpGasPrice {
    /// New gas price of the transaction.
    #[structopt(long, parse(try_from_str = parse_u256))]
    gas_price: U256,
    /// Hex-encoded RLP of the transaction.
    #[structopt(long)]
    rlp: HexBytes,
    /// Path to a JSON key file.
    #[structopt(long)]
    key_path: std::path::PathBuf,
    /// An RPC endpoint to send the transaction to.
    #[structopt(long)]
    rpc: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "transfer", about = "Create a signed transfer transaction.")]
struct Transfer {
    /// Destination address.
    #[structopt(long)]
    to: H160,
    /// Transaction nonce.
    #[structopt(
        long,
        parse(try_from_str = parse_u256),
        required_if("rpc", "None")
    )]
    nonce: Option<U256>,
    /// Value to transfer.
    #[structopt(long, parse(try_from_str = parse_u256))]
    amount: U256,
    /// Gas Price to use.
    #[structopt(long, parse(try_from_str = parse_u256), default_value = "20_000_000_000")] 
    gas_price: U256,
    /// Chain ID used for signing.
    #[structopt(long, default_value = "105")]
    chain_id: u64,
    /// Path to a JSON key file.
    #[structopt(long)]
    key_path: std::path::PathBuf,
    /// An RPC endpoint to send the transaction to.
    #[structopt(long)]
    rpc: Option<String>,
}

fn main() -> Result<(), String> {
    env_logger::init();
    let opt = Opt::from_args();
    let result = match opt {
        Opt::BumpGasPrice(opt) => {
            let rlp = bump_gas_price::bump_gas_price(opt.gas_price, &opt.rlp, &opt.key_path.as_ref())?;
            let r = format!("RLP: {}", rlp.to_hex::<String>());
            if let Some(url) = opt.rpc {
                println!("Submitting {} to {}", r, url);
                let r = crate::web3::send(&url, |rt, web3| rt.block_on(
                    web3.eth().send_raw_transaction(rlp.clone().into())
                )).map_err(crate::utils::debug)?;
                println!("Response: {:?}", r);
            }
            r
        },
        Opt::Transfer(opt) => {
            let nonce = transfer::get_nonce(&opt.rpc, &opt.nonce, &opt.key_path)?;
            let rlp = transfer::transfer(
                opt.to,
                nonce,
                opt.amount,
                opt.gas_price,
                opt.chain_id,
                &opt.key_path.as_ref()
            )?;
            let r = format!("RLP: {}", rlp.to_hex::<String>());
            if let Some(url) = opt.rpc {
                println!("Submitting {} to {}", r, url);
                let r = crate::web3::send(&url, |rt, web3| rt.block_on(
                    web3.eth().send_raw_transaction(rlp.clone().into())
                )).map_err(crate::utils::debug)?;
                println!("Response: {:?}", r);
            }
            r
        },
    };
    println!("{}", result);
    Ok(())
}

pub struct HexBytes(Vec<u8>);

impl std::ops::Deref for HexBytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &*self.0
    }   
}

impl std::fmt::Debug for HexBytes {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s: String = self.0.to_hex();
        write!(fmt, "{}", s)
    }
}

impl std::str::FromStr for HexBytes {
    type Err = rustc_hex::FromHexError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HexBytes(s.from_hex::<Vec<u8>>()?))
    }
}

fn parse_u256(s: &str) -> Result<U256, <U256 as std::str::FromStr>::Err> {
    let s = s.replace('_', "");
    let num: Option<u64> = s.parse().ok();
    if let Some(num) = num {
        Ok(num.into())
    } else {
        s.parse()
    }
}
