use diesel::prelude::*;
use ipnet::IpNet;

use crate::schema::prefixes;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = prefixes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Prefix {
    pub id: i32,
    pub network: IpNet,
    pub vlan_id: Option<i32>,
    pub status: String,
    pub description: Option<String>,
}

// Ohne `status`: Die Datenbank setzt den Default 'active'.
#[derive(Insertable)]
#[diesel(table_name = prefixes)]
pub struct NewPrefix<'a> {
    pub network: IpNet,
    pub vlan_id: Option<i32>,
    pub description: Option<&'a str>,
}

pub fn create(
    conn: &mut PgConnection,
    network: IpNet,
    vlan_id: Option<i32>,
    description: Option<&str>,
) -> QueryResult<Prefix> {
    diesel::insert_into(prefixes::table)
        .values(NewPrefix { network, vlan_id, description })
        .returning(Prefix::as_returning())
        .get_result(conn)
}

pub fn find(conn: &mut PgConnection, id: i32) -> QueryResult<Prefix> {
    prefixes::table
        .find(id)
        .select(Prefix::as_select())
        .first(conn)
}

pub fn list(conn: &mut PgConnection) -> QueryResult<Vec<Prefix>> {
    prefixes::table
        .order(prefixes::network.asc())
        .select(Prefix::as_select())
        .load(conn)
}