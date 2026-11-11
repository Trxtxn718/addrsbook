use std::net::Ipv4Addr;

pub struct IpAddress {
    name: String,
    ipv4addr: Ipv4Addr,
    status: IpAddressStatus,
    description: String,
}

enum IpAddressStatus{
    Active,
    Inactive,
    Unknown,
}

impl IpAddress {
    pub fn new(name:String, ipv4addr: Ipv4Addr, description: String) -> IpAddress {
        IpAddress{
            name,
            ipv4addr,
            status: IpAddressStatus::Active,
            description,
        }

    }
    pub fn change_status(&mut self, status: IpAddressStatus) {
        self.status = status;
    }
}