use std::net::IpAddr;

use diesel::prelude::*;
use ipnet::IpNet;

use crate::prefix;
use crate::schema::ip_addresses;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = ip_addresses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IpAddress {
    pub id: i32,
    pub address: IpNet,
    pub prefix_id: i32,
    pub hostname: Option<String>,
    pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = ip_addresses)]
pub struct NewIpAddress<'a> {
    pub address: IpNet,
    pub prefix_id: i32,
    pub hostname: Option<&'a str>,
}

#[derive(Debug)]
pub enum AddressError {
    NotInPrefix,
    Database(diesel::result::Error),
}

impl From<diesel::result::Error> for AddressError {
    fn from(e: diesel::result::Error) -> Self {
        AddressError::Database(e)
    }
}

pub fn create(
    conn: &mut PgConnection,
    address: IpAddr,
    prefix_id: i32,
    hostname: Option<&str>,
) -> Result<IpAddress, AddressError> {
    // Validierung: Die IP muss im angegebenen Prefix liegen.
    let pfx = prefix::find(conn, prefix_id)?;
    if !pfx.network.contains(&address) {
        return Err(AddressError::NotInPrefix);
    }

    let row = diesel::insert_into(ip_addresses::table)
        .values(NewIpAddress {
            address: IpNet::from(address),
            prefix_id,
            hostname,
        })
        .returning(IpAddress::as_returning())
        .get_result(conn)?;
    Ok(row)
}

pub fn list_in_prefix(conn: &mut PgConnection, prefix_id: i32) -> QueryResult<Vec<IpAddress>> {
    ip_addresses::table
        .filter(ip_addresses::prefix_id.eq(prefix_id))
        .order(ip_addresses::address.asc())
        .select(IpAddress::as_select())
        .load(conn)
}

pub fn list(conn: &mut PgConnection) -> QueryResult<Vec<IpAddress>> {
    ip_addresses::table
        .order(ip_addresses::address.asc())
        .select(IpAddress::as_select())
        .load(conn)
}