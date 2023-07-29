extern crate pnet;

use pnet::datalink;
use std::io;

pub fn get() -> Result<String, io::Error> {
    let interfaces = datalink::interfaces();
    let ethernet_interfaces: Vec<_> = interfaces
        .into_iter()
        .filter(|interface| interface.is_up() && !interface.ips.is_empty())
        .collect();

    for interface in ethernet_interfaces {
        for ip_network in interface.ips {
            if ip_network.is_ipv4() {
                let ip_address = format!("{}", ip_network.ip());
                if ip_address == "127.0.0.1" {
                    continue;
                };
                return Ok(ip_address);
            }
        }
    }
    return Err(io::Error::new(
        io::ErrorKind::Other,
        "Failed to get ip address",
    ));
}
