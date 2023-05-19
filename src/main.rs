use ipnet::Ipv4Net;
use std::sync::{Mutex, Arc};
use threadpool::ThreadPool;
use std::process::Stdio;

mod utils;

fn main() {

    let pool: ThreadPool = ThreadPool::new(512);
    let (count, count_un) = scan_network("10.0.0.0/21", &pool);
    let cuenta_inactivos = count_un.len();

    println!("[+] There are {count} active hosts.");
    println!("[+] There are {cuenta_inactivos} unactive hosts");


}

fn fping(ip: &str) -> Result<bool, String> {

    let status = std::process::Command::new("fping")
        .arg("-t 1000")
        .arg("-c 3")
        .arg("-q")
        .arg(ip)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success(){
        Ok(true)
    } else {
        Ok(false)
    }

}

fn ping(ip: &str) -> Result<bool, String> {

    let status = std::process::Command::new("ping")
        .stdout(Stdio::null())
        .arg("-W 30")
        .arg("-c 10")
        .arg("-t 255")
        .arg(ip)
        .status()
        .map_err(|e| e.to_string())?;
    
    if status.success(){
        Ok(true)
    } else {
        Ok(false)
    }
} 

fn scan_network(net_str: &str, pool: &ThreadPool) -> (usize, Vec<String>) {

    let net: Ipv4Net = net_str.parse().unwrap();
    let active_hosts = Arc::new(Mutex::new(0));
    let unactive_hosts: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    for host in net.hosts() {
        let active_hosts = Arc::clone(&active_hosts);
        let unactives = Arc::clone(&unactive_hosts);

        let pool = pool.clone();

        pool.execute(move || {
            let ip = host.to_string();
            match ping(&ip) {
                Ok(true) => {
                    println!("{ip} is up.");
                    let mut count = active_hosts.lock().unwrap();
                    *count += 1;
                }
                Ok(false) => {
                    let ip2 = ip.clone();
                    unactives.lock().unwrap().push(ip2);
                }
                Err(_) => {}
            }
        });
    }

    pool.join();

    let active_count = *active_hosts.lock().unwrap();
    let unactive_list = unactive_hosts.lock().unwrap().clone();

    (active_count, unactive_list)
}

