use ipnet::Ipv4Net;
use spinners::{Spinner, Spinners};
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence, ICMP};
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::time::Duration;
use rand::random;
use tokio::time;
use colored::Colorize;
use libarp::{arp::ArpMessage, client::ArpClient, interfaces::Interface, interfaces::MacAddr};

async fn ping(client: Client, 
    addr: IpAddr, 
    retries: u16, 
    timeout: u64) -> (bool, String){

    let payload = [0; 56];
    let mut pinger = client.pinger(addr, PingIdentifier(random())).await;
    pinger.timeout(Duration::from_secs(timeout));
    let mut interval = time::interval(Duration::from_secs(1));

    for idx in 0 .. retries{

        interval.tick().await;
        match pinger.ping(PingSequence(idx), &payload).await {

            Ok((IcmpPacket::V4(_packet), _dur)) => { 
                
                /*println!(
                "No.{}: {} bytes from {}: icmp_seq={} ttl={:?} time={:0.2?}",
                idx,
                packet.get_size(),
                packet.get_source(),
                packet.get_sequence(),
                packet.get_ttl(),
                dur
            );*/

            return (true, addr.to_string());
        },
            Ok((IcmpPacket::V6(packet), dur)) => { 

                println!(
                "No.{}: {} bytes from {}: icmp_seq={} hlim={} time={:0.2?}",
                idx,
                packet.get_size(),
                packet.get_source(),
                packet.get_sequence(),
                packet.get_max_hop_limit(),
                dur
            );

            return (true, addr.to_string());
        },

            Err(_e) => {},
        };
    }
    return (false, "nada".to_string());
}


pub async fn scan(
    network: &str, 
    retries: u16,
    timeout: u64
) -> Result<Vec<String>, Box<dyn std::error::Error>>{

    let net :Ipv4Net = network.parse().unwrap();
    let client_v4 = Client::new(&Config::default())?;
    let client_v6 = Client::new(&Config::builder().kind(ICMP::V6).build())?;
    let mut tasks = Vec::new();
    let mut responsive_hosts: Vec<String> = Vec::new();


    for host in net.hosts() {

        let ip = host.to_string();

        match ip.parse() {

            Ok(IpAddr::V4(addr)) => { // Call ping with IPV4
                tasks.push(tokio::spawn(ping(
                    client_v4.clone(), 
                    IpAddr::V4(addr),
                    retries, 
                    timeout
                )))
            }
            Ok(IpAddr::V6(addr)) => { // Call ping with IPV6
                tasks.push(tokio::spawn(ping(
                    client_v6.clone(),
                    IpAddr::V6(addr),
                    retries, 
                    timeout
                )))
            }// Not valid
            Err(e) => println!("{} Parse to IpAddr error: {}", ip, e),
        }
        
    }

    for task in tasks{

        match task.await{
            Ok((true, st)) => {
                let host = st.to_string();
                responsive_hosts.push(host);
            }
            Ok((false, _st)) => {}
            Err(_) => {}
        }
    }
    //join_all(tasks).await;

    Ok(responsive_hosts)
}


async fn resolve_simple(ip: &str) {

    let ip_addr = Ipv4Addr::from_str(ip).unwrap();
    let mut client = ArpClient::new().unwrap();

    let result = client.ip_to_mac(ip_addr, None);
    match result.await{
        Ok(addr) => {
            println!("Ip {} has MAC {}",
            ip_addr.to_string(),
            addr
        );
        },
        Err(_) => println!("Could not get MAC")
    }
}

#[tokio::main]
pub async fn run(network: String, retries: u16, timeout: u64) {

    let text = "Scanning network...".green();
    let mut sp = Spinner::new(Spinners::BouncingBar, text.to_string());
    let scan = scan(&network, retries, timeout).await;    
    sp.stop_with_newline();

    match scan{
        Ok(vec) => {
            println!(
                "{} {} {}",
                "[+] There are".yellow(),
                 vec.len().to_string().green(),
                 "active hosts. (ICMP only)".yellow()
                );

            for e in vec{
                println!("-> {}", e);
            }
        },
        Err(_) => {}
        
    }
}