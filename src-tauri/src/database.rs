use diesel::prelude::{MysqlConnection, Connection};
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

pub fn establish_connection() -> MysqlConnection {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  MysqlConnection::establish(&database_url)
      .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let manager = ConnectionManager::<MysqlConnection>::new(database_url);
  let pool = Pool::builder().build(manager).expect("Failed to create pool.");
  return pool
}
