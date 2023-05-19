use clap::Parser;
mod utils;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    network: String,

    #[arg(short, long, default_value_t = 1)]
    retries: u16,

    #[arg(short, long, default_value_t = 1)]
    timeout: u64,
}
fn main() {

    let args = Args::parse();

    utils::run(
        args.network,
        args.retries,
        args.timeout
    );
}   