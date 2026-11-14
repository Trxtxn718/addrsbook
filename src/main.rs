mod schema;
mod vlan;

use diesel::prelude::*;

fn main() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL fehlt");
    let mut conn = PgConnection::establish(&url).expect("Verbindung fehlgeschlagen");

    let neu = vlan::create(&mut conn, 20, "Server").expect("Insert fehlgeschlagen");
    println!("Angelegt: {neu:?}");

    for v in vlan::list(&mut conn).expect("Query fehlgeschlagen") {
        println!("VLAN {} ({})", v.vid, v.name);
    }
}