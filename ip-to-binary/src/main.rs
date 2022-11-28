use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ip = &args[1];
    let octets: Vec<&str> = ip.split(".").collect();

    let network_mask = args[2].parse::<usize>().unwrap();

    let mut binary: Vec<String> = Vec::new();
    for octet in octets {
        let octet = octet.parse::<u8>().unwrap();
        let octet = format!("{:08b}", octet);
        binary.push(octet);
    }

    let mut binary_ip = binary.join(".") + "\n";
    let network_len = String::from_utf8(vec![b'-'; network_mask + 3]).unwrap();
    
    binary_ip.push_str(&(network_len + "\n"));
    binary_ip.push_str(&(ip.to_owned() + "/" + &(network_mask.to_string()) + "\n"));

    let mut file = File::create("data/data.txt").unwrap();
    file.write_all(binary_ip.as_bytes()).unwrap();
}
