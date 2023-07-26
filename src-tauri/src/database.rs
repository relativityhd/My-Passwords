use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::Error;
// use dotenvy::dotenv;
// use std::env;

pub async fn establish_connection() -> Result<Surreal<Client>, Error> {
  // dotenv().ok();
  // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

  // Signin as a namespace, database, or root user
  db.signin(Root {
    username: "root",
    password: "root",
  })
  .await?;
  db.use_ns("dev").use_db("my-accounts").await?;
  Ok(db)
}
