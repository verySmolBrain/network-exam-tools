use std::fs::File;
use std::io:: { BufRead, BufReader };
use std::env;

struct Interface {
    prefix: String,
    mask: usize,
    ip: String,
}

fn main() {
    let file = File::open("data/interface").unwrap();
    let mut interfaces: Vec<Interface> = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let args: Vec<&str> = line.split_whitespace().collect();

        let ip = args[0];
        let network_mask = args[1].parse::<usize>().unwrap();

        let octets: Vec<&str> = ip.split(".").collect();

        let mut binary: Vec<String> = Vec::new();
        for octet in octets {
            let octet = octet.parse::<u8>().unwrap();
            let octet = format!("{:08b}", octet);
            binary.push(octet);
        }

        let interface = Interface {
            prefix: binary.join(""),
            mask: network_mask,
            ip: ip.to_string(),
        };

        interfaces.push(interface);
    }

    let ip = env::args().nth(1).unwrap();
    let octets: Vec<&str> = ip.split(".").collect();

    let mut binary: Vec<String> = Vec::new();
    for octet in octets {
        let octet = octet.parse::<u8>().unwrap();
        let octet = format!("{:08b}", octet);
        binary.push(octet);
    }
    let ip = binary.join("");

    let mut matches: Vec<Interface> = Vec::new();

    for interface in interfaces {
        let prefix = &interface.prefix[..interface.mask];
        let ip = &ip[..interface.mask];

        if prefix == ip {
            matches.push(interface);
        }
    }

    matches.sort_by(|a, b| b.mask.cmp(&a.mask));
    for interface in &matches {
        println!("{} {}", interface.ip, interface.mask);
    }

    if matches.len() == 0 {
        println!("No match");
    }
}
