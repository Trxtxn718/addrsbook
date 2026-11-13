use ipnet::Ipv4Net;

use crate::IpAddress;

pub struct IpNetwork{
    name: String,
    description: String,
    ipv4net: Ipv4Net,
    managed_addresses: Vec<IpAddress>
}

impl IpNetwork {
    pub fn new(name: String, description: String, ipv4net: Ipv4Net) -> IpNetwork{
        IpNetwork { name, description, ipv4net, managed_addresses: Vec::new() }
    }

    pub fn add_new_address(&mut self, address: IpAddress){
        self.managed_addresses.push(address);
    }
}