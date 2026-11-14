mod schema;
mod vlan;

use diesel::prelude::*;

fn main() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing!");
    let mut conn = PgConnection::establish(&url).expect("Connection failed");

    let newVlan = vlan::create(&mut conn, 20, "Server").expect("Insert failed");
    println!("Angelegt: {newVlan:?}");

    for v in vlan::list(&mut conn).expect("Error with query") {
        println!("VLAN {} ({})", v.vid, v.name);
    }
}