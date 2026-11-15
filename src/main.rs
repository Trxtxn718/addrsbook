mod schema;
mod vlan;
mod prefix;
use diesel::prelude::*;
use ipnet::IpNet;

fn main() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing!");
    let mut conn = PgConnection::establish(&url).expect("Connection failed");


    // VLAN testing:
    let new_vlan = vlan::create(&mut conn, 20, "Server").expect("Insert failed");
    println!("Angelegt: {new_vlan:?}");

    for v in vlan::list(&mut conn).expect("Error with query") {
        println!("VLAN {} ({})", v.vid, v.name);
    }

    // Prefix testing:
    let test_net: IpNet = "10.0.0.0/24".parse().unwrap();
    let new_prefix = prefix::create(&mut conn, test_net, None, Some("Server"));
    

}