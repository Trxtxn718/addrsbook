use std::net::Ipv4Addr;

use ipnet::Ipv4Net;

use crate::{ip_address::IpAddress, ip_network::IpNetwork, ip_status::IpStatus};

mod ip_address;
mod ip_network;
pub mod ip_status;

fn main() {

    let network = Ipv4Net::new(Ipv4Addr::new(10, 1, 1, 0), 24).unwrap();

    let mut test_net = IpNetwork::new(String::from("Test Netz"), String::from("Test Netz"), network);


    let mut ip = IpAddress::new(String::from("Test"), Ipv4Addr::new(127, 0, 0, 1).into(), String::from("This is a test"));
    ip.change_status(IpStatus::Inactive);
    println!("Hello, world!");
}
