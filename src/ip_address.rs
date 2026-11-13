use std::net::Ipv4Addr;
use crate::ip_status::IpStatus;

pub struct IpAddress {
    name: String,
    ipv4addr: Ipv4Addr,
    status: IpStatus,
    description: String,
}



impl IpAddress {
    pub fn new(name:String, ipv4addr: Ipv4Addr, description: String) -> IpAddress {
        IpAddress{
            name,
            ipv4addr,
            status: IpStatus::Active,
            description,
        }

    }
    pub fn change_status(&mut self, status: IpStatus) {
        self.status = status;
    }
}