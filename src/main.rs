use std::net::Ipv4Addr;
pub use crate::ip_address::{IpAddress};
use crate::ip_status::IpStatus;

mod ip_address;
mod ip_network;
pub mod ip_status;

fn main() {
    let mut ip = IpAddress::new(String::from("Test"), Ipv4Addr::new(127, 0, 0, 1).into(), String::from("This is a test"));
    ip.change_status(IpStatus::Inactive);
    println!("Hello, world!");
}
