use structopt::StructOpt;

mod bump_gas_price;
mod utils;

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
    #[structopt(long)]
    gas_price: String,
    /// Hex-encoded RLP of the transaction.
    #[structopt(long)]
    rlp: String,
    /// Path to a JSON key file.
    #[structopt(long)]
    key_path: std::path::PathBuf,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "transfer", about = "Create a signed transfer transaction.")]
struct Transfer {
    /// Destination
    #[structopt(long)]
    to: String,
    /// Nonce
    #[structopt(long)]
    nonce: String,
    /// Value
    #[structopt(long)]
    amount: String,
    /// Path to a JSON key file.
    #[structopt(long)]
    key_path: std::path::PathBuf,
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();
    let result = match opt {
        Opt::BumpGasPrice(opt) => {
            bump_gas_price::bump_gas_price(&opt.gas_price, &opt.rlp, &opt.key_path.as_ref())?
        },
        Opt::Transfer(opt) => {
            todo!()
        },
    };
    println!("{}", result);
    Ok(())
}
