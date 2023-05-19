use ipnet::Ipv4Net;
use std::sync::{Mutex, Arc};
use threadpool::ThreadPool;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence, ICMP};
use futures::future::join_all;
use std::net::IpAddr;
use std::time::Duration;
use rand::random;
use tokio::{time, task};

// Utilities module (ping, MAC detection, IP duplicity det, formatting, etc)
pub struct Args{
    pub network: String,
    pub timeout: i32,
    pub get_mac: bool
}


async fn ping(client: Client, addr: IpAddr) -> (bool, String){

    let payload = [0; 56];
    let mut pinger = client.pinger(addr, PingIdentifier(random())).await;
    pinger.timeout(Duration::from_secs(1));
    let mut interval = time::interval(Duration::from_secs(1));

    for idx in 0..4 { //Retrys

        interval.tick().await;
        match pinger.ping(PingSequence(idx), &payload).await {

            Ok((IcmpPacket::V4(packet), dur)) => { 
                
                println!(
                "No.{}: {} bytes from {}: icmp_seq={} ttl={:?} time={:0.2?}",
                idx,
                packet.get_size(),
                packet.get_source(),
                packet.get_sequence(),
                packet.get_ttl(),
                dur
            );

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

            Err(e) => {},
        };
    }
    return (false, "nada".to_string());
    //println!("[+] {} done.", pinger.host);
}


pub async fn scan(network: &str) -> Result<(), Box<dyn std::error::Error>>{

    let net :Ipv4Net = network.parse().unwrap();
    let client_v4 = Client::new(&Config::default())?;
    let client_v6 = Client::new(&Config::builder().kind(ICMP::V6).build())?;
    let mut tasks = Vec::new();
    let mut responsive_hosts: Vec<String> = Vec::new();


    for host in net.hosts() {

        let ip = host.to_string();

        match ip.parse() {

            Ok(IpAddr::V4(addr)) => { // Call ping with IPV4
                tasks.push(tokio::spawn(ping(client_v4.clone(), IpAddr::V4(addr))))
            }
            Ok(IpAddr::V6(addr)) => { // Call ping with IPV6
                tasks.push(tokio::spawn(ping(client_v6.clone(), IpAddr::V6(addr))))
            }// Not valid
            Err(e) => println!("{} parse to ipaddr error: {}", ip, e),
        }
        
    }

    for task in tasks{

        match task.await{
            Ok((true, st)) => {
                let host = st.to_string();
                responsive_hosts.push(host);
            }
            Ok((false, st)) => {}
            Err(_) => {}
        }
    }
    println!("{:?}", responsive_hosts.len());
    //join_all(tasks).await;

    Ok(())
}

fn get_mac() -> String{ // Return MAC as string
    todo!()
}

#[tokio::main]
pub async fn run() {
    let _ = scan("10.0.0.0/21").await;
}