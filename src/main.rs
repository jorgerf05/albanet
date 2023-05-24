use clap::Parser;
mod utils;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Network to be scanned in CIDR notation.
    #[arg(short, long)]
    network: String,

    /// Number of retries. Default is 1.
    #[arg(short, long, default_value_t = 1)]
    retries: u16,

    /// Timeout in seconds. Default is 1.
    #[arg(short, long, default_value_t = 1)]
    timeout: u64,

    /// Path to JSON config file.
    #[arg(short, long)]
    json: Option<String>,
}
fn main() {

    let args = Args::parse();

    if args.json.is_some(){
        //parse json
    }
    else {
        utils::run(
            args.network,
            args.retries,
            args.timeout
        );
    }
}   