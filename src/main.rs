use ipnet::Ipv4Net;
use std::sync::{Mutex, Arc};

fn main() {

    let net: Ipv4Net = "192.168.100.0/16".parse().unwrap();
    let mut threads = vec![];
    let active_hosts = Arc::new(Mutex::new(0));

    for host in net.hosts() {

        let active_hosts = Arc::clone(&active_hosts);

        let t = std::thread::spawn(move || {

            let ip = &host.to_string();

            match ping(ip) {
                Ok(true) => {
                    println!("{ip} is up.");
                    let mut count = active_hosts.lock().unwrap();
                    *count += 1;
                }
                Ok(false) | Err(_) => {}
            }
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }

    let count = active_hosts.lock().unwrap();
    println!("[+] There are {count} active hosts.");
}


fn ping(ip: &str) -> Result<bool, String> {
    let status = std::process::Command::new("fping")
        .arg("-t 1000")
        .arg("-q")
        .arg("-r 0")
        .arg(ip)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success(){
        Ok(true)
    } else {
        Ok(false)
    }
} 

