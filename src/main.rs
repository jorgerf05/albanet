use clap::Parser;
use chrono;
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

    let dt = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
    let t0 = std::time::Instant::now();
    println!(
        "ALBANET v1.0.0\nStarting scan at {}",
        dt
    );

    let args = Args::parse();
    if args.json.is_some(){
        //TODO: parse json
    }
    else {
        utils::run(
            args.network,
            args.retries,
            args.timeout
        );
    }
    
    let t1 = std::time::Instant::now();
    let elapsed = t1.duration_since(t0).as_secs_f32();
    println!(
        "\nScan completed in {:.2} seconds.",
        elapsed
    )
}   