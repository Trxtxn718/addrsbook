use diesel::prelude::*;

use crate::schema::vlans;

/// Eine Zeile aus der Tabelle `vlans` (Lesen).
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = vlans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Vlan {
    pub id: i32,
    pub vid: i32,
    pub name: String,
}

/// Daten für einen neuen Eintrag (Einfügen, ohne `id`).
#[derive(Insertable)]
#[diesel(table_name = vlans)]
pub struct NewVlan<'a> {
    pub vid: i32,
    pub name: &'a str,
}

pub fn create(conn: &mut PgConnection, vid: i32, name: &str) -> QueryResult<Vlan> {
    diesel::insert_into(vlans::table)
        .values(NewVlan { vid, name })
        .returning(Vlan::as_returning())
        .get_result(conn)
}

pub fn list(conn: &mut PgConnection) -> QueryResult<Vec<Vlan>> {
    vlans::table
        .order(vlans::vid.asc())
        .select(Vlan::as_select())
        .load(conn)
}