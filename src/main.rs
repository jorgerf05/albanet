use ipnet::Ipv4Net;
use std::sync::{Mutex, Arc};
use threadpool::ThreadPool;

fn main() {

    //TODO: Get CLI args
    //TODO: Get JSON config file
    let net: Ipv4Net = "10.0.0.0/21".parse().unwrap();
    let active_hosts = Arc::new(Mutex::new(0));
    let unactive_ips = Arc::new(Mutex::new(vec![""]));
    let pool = ThreadPool::new(1024);

    for host in net.hosts() {

        let active_hosts = Arc::clone(&active_hosts);
        let unactive_ips = Arc::clone(&unactive_ips);

        pool.execute(move || {

            let ip = host.to_string();

            match ping(&ip) {
                Ok(true) => {
                    println!("{ip} is up.");
                    let mut count = active_hosts.lock().unwrap();
                    *count += 1;
                }
                Ok(false) => {
                    let mut unactives = unactive_ips.lock().unwrap();

                }
                
                Err(_) => {}
            }
        });
    }

    pool.join();

    let count = active_hosts.lock().unwrap();
    println!("[+] There are {count} active hosts.");
}



fn fping(ip: &str) -> Result<bool, String> {

    let status = std::process::Command::new("fping")
        .arg("-t 20000")
        .arg("-q")
        .arg("-r 10")
        .arg("-c 10")
        .arg(ip)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success(){
        Ok(true)
    } else {
        Ok(false)
    }

}

fn ping<'a>(ip: &'a str) -> Result<bool, String> {

    let status = std::process::Command::new("ping")
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


