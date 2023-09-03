use colored::Colorize;
use ipnet::Ipv4Net;
use rand::random;
use spinners::{Spinner, Spinners};
use std::net::IpAddr;
use std::time::Duration;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence, ICMP};
use tokio::time;

// Main ping function.
async fn ping(client: Client, addr: IpAddr, retries: u16, timeout: u64) -> (bool, String) {
    let payload = [0; 56];
    let mut pinger = client.pinger(addr, PingIdentifier(random())).await;
    pinger.timeout(Duration::from_secs(timeout));
    let mut interval = time::interval(Duration::from_secs(1));

    for idx in 0..retries {
        interval.tick().await;
        match pinger.ping(PingSequence(idx), &payload).await {
            Ok((IcmpPacket::V4(_packet), _dur)) => {
                return (true, addr.to_string());
            }

            Ok((IcmpPacket::V6(packet), dur)) => {
                return (true, addr.to_string());
            }

            Err(_e) => {}
        };
    }
    return (false, "nada".to_string());
}

pub async fn scan(
    network: &str,
    retries: u16,
    timeout: u64,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let net: Ipv4Net = network.parse().unwrap();
    let client_v4 = Client::new(&Config::default())?;
    let client_v6 = Client::new(&Config::builder().kind(ICMP::V6).build())?;
    let mut tasks = Vec::new();
    let mut responsive_hosts: Vec<String> = Vec::new();

    for host in net.hosts() {
        let ip = host.to_string();
        match ip.parse() {
            Ok(IpAddr::V4(addr)) => {
                // Call ping with IPV4
                tasks.push(tokio::spawn(ping(
                    client_v4.clone(),
                    IpAddr::V4(addr),
                    retries,
                    timeout,
                )))
            }
            Ok(IpAddr::V6(addr)) => {
                // Call ping with IPV6
                tasks.push(tokio::spawn(ping(
                    client_v6.clone(),
                    IpAddr::V6(addr),
                    retries,
                    timeout,
                )))
            } // Not valid
            Err(e) => println!("{} Parse to IpAddr error: {}", ip, e),
        }
    }

    // Now we'll wait for every thread and capture its output
    for task in tasks {
        match task.await {
            // If we got a live host
            Ok((true, st)) => {
                let host = st.to_string();
                responsive_hosts.push(host);
            }
            // Else
            Ok((false, _)) => {}
            Err(_) => {}
        }
    }
    // And lastly, we return the live hosts vector
    Ok(responsive_hosts)
}

#[tokio::main]
pub async fn run(network: String, retries: u16, timeout: u64) {
    let text = "Scanning network...".green();
    let mut sp = Spinner::new(Spinners::BouncingBar, text.to_string());
    let scan_results = scan(&network, retries, timeout).await;
    sp.stop_with_newline();

    match scan_results {
        Ok(pos_results) => {
            println!(
                "{} {} {}", // This line is necessary because of the 3 colors used
                "[+] Found".yellow(),
                pos_results.len().to_string().green(),
                "active hosts. (ICMP only)".yellow()
            );

            for e in pos_results {
                println!("-> {}", e);
            }
        }
        Err(_) => {}
    }
}
