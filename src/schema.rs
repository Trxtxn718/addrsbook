// @generated automatically by Diesel CLI.

diesel::table! {
    ip_addresses (id) {
        id -> Int4,
        address -> Inet,
        prefix_id -> Int4,
        hostname -> Nullable<Text>,
        status -> Text,
    }
}

diesel::table! {
    prefixes (id) {
        id -> Int4,
        network -> Cidr,
        vlan_id -> Nullable<Int4>,
        status -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    vlans (id) {
        id -> Int4,
        vid -> Int4,
        name -> Text,
    }
}

diesel::joinable!(ip_addresses -> prefixes (prefix_id));
diesel::joinable!(prefixes -> vlans (vlan_id));

diesel::allow_tables_to_appear_in_same_query!(ip_addresses, prefixes, vlans,);
