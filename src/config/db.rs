#[allow(unused_imports)]
use diesel::{
    r2d2::{self, ConnectionManager},
};
use log::info;
#[cfg(test)]
use diesel::sqlite::SqliteConnection;

#[cfg(test)]
pub type Connection = SqliteConnection;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;


#[cfg(test)]
pub fn migrate_and_config_db(url: &str) -> Pool {
    use diesel::{sql_query, RunQueryDsl};

    info!("Migrating and configuring database...");
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    sql_query(r#"DROP TABLE IF EXISTS resources;"#).execute(&pool.get().unwrap()).unwrap();

    pool
}

