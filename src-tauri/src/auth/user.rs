use diesel::prelude::*;

use crate::schema::users;

use super::error::AuthError;
use super::cookie::Auth;
use super::types::*;

#[derive(Queryable)]
pub struct User {
  pub id: u32,
  pub username: String,
  pub pass: String
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser<'a> {
  username: &'a str,
  pass: &'a str
}

impl<'a> From<&'a Auth> for NewUser<'a> {
  fn from(auth: &'a Auth) -> Self {
    NewUser { username: &auth.username, pass: &auth.pass }
  }
}

impl Into<Auth> for &User {
  fn into(self) -> Auth {
      Auth { username: String::from(&self.username), pass: String::from(&self.pass) }
  }
}

impl User {
  pub fn from_username(connection_pool: &DBConnectionPool, username: &String) -> Result<User, AuthError> {
    let connection = &mut connection_pool.get()?;

    match users::table
      .select(users::all_columns)
      .filter(users::username.eq(username))
      .first::<User>(connection) {
        Ok(user) => Ok(user),
        Err(diesel::NotFound) => Err(AuthError::UnknownUsername),
        Err(e) => Err(AuthError::FailedQuery(e, "find user by username"))
    }
  }

  pub fn check_username(connection_pool: &DBConnectionPool, username: &String) -> Result<bool, AuthError> {
    let connection = &mut connection_pool.get()?;
    match users::table
      .select(users::all_columns)
      .filter(users::username.eq(username))
      .first::<User>(connection) {
        Ok(_) => Ok(true),
        Err(diesel::NotFound) => Ok(false),
        Err(e) => Err(AuthError::FailedQuery(e, "checking username"))
    }
  }

  pub fn create_user(connection_pool: &DBConnectionPool, username: &String, pass: &String) -> Result<User, AuthError> {
    let connection = &mut connection_pool.get()?;

    let auth = Auth {username: String::from(username), pass: String::from(pass)};
    let new_user: NewUser = (&auth).into();
    // Need two seperate queries since MySQL doesn't support the RETURNING * statement, MariaDB however does but Diesel has no MariaDB Backend implemented yet: https://github.com/diesel-rs/diesel/issues/2535#issuecomment-1292446824
    diesel::insert_into(users::table)
      .values(&new_user)
      .execute(connection)
      .map_err(|e| AuthError::FailedQuery(e, "inserting new user"))?;

    let user: User = users::table
      .select(users::all_columns)
      .filter(users::username.eq(username))
      .first(connection)
      .map_err(|e| AuthError::FailedQuery(e, "find inserted user by username"))?;
    Ok(user)
  }
}
