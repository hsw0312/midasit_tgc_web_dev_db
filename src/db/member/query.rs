use crate::db::member::model::TableMember;
use mysql::{params, prelude::*};

pub fn select_members(conn: &mut mysql::PooledConn) -> mysql::error::Result<Vec<TableMember>> {
    conn.query_map(
        r"
        SELECT * FROM `hsw_tgc`.`hsw0312` LIMIT 1000;
        ",
        |(my_name, my_age)| TableMember {
            name: my_name,
            age: my_age,
        },
    )
}

pub fn post_member(
    conn: &mut mysql::PooledConn,
    name: String,
    age: i32,
) -> mysql::error::Result<()> {
    conn.exec_drop(
        r"
        INSERT INTO `hsw_tgc`.`hsw0312` (name, age) VALUES (:name, :age);
        ",
        params! {
            "name" => name,
            "age" => age,
        },
    )
}
