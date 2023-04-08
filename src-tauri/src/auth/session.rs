use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use rand_core::OsRng;

use super::cookie::{Cookie, Auth};
use super::error::AuthError;
use super::user::User;
use super::types::*;

pub struct Session<'a> {
  id: Option<u32>,
  username: Option<String>,
  cookie: Cookie,
  hasher: Hasher<'a>,
  connection_pool: DBConnectionPool
}

impl<'a> Session<'a> {

  pub fn new(connection_pool: DBConnectionPool, cookie: Cookie, hasher: Hasher) -> Session {
    let mut session = Session { id: None, username: None, cookie, connection_pool, hasher };

    if let Some(auth) = (&session.cookie).read().expect("Read to be bulletproof") {
      if let Ok(user) = User::from_username(&session.connection_pool, &auth.username) {
        if auth.pass == user.pass {
          session.update_session(user.id, user.username)
        }
      }
    };

    session
  }

  fn update_session(&mut self, id: u32, username: String) {
      self.id = Some(id);
      self.username = Some(username);
  }

  pub fn login(&mut self, username: String, pass: String, save_cookie: bool) -> Result<(), AuthError> {

    let user = User::from_username(&self.connection_pool, &username)?;

    let parsed_hash = PasswordHash::new(&user.pass)?;
    //.map_err(|error| { format!("Can't parse hashed password {}\n{}", pass, error.to_string())});

    let successful_login = self.hasher.verify_password(pass.as_bytes(), &parsed_hash).is_ok();

    if !successful_login {
      return Err(AuthError::InvalidPassword);
    }

    if save_cookie {
      let auth: Auth = (&user).into();
      self.cookie.write(&auth).expect("Write to be working, however not relevant for app to work...");
    }

    self.update_session(user.id, user.username);
    Ok(())
  }

  pub fn logout(&mut self) -> Result<(), AuthError> {
    self.id = None;
    self.username = None;
    // Delete cookie
    self.cookie.delete()?;

    Ok(())
  }

  pub fn signup(&mut self, username: String, pass: String, save_cookie: bool) -> Result<bool, AuthError> {
    // Check if username already exists
    if User::check_username(&self.connection_pool, &username).expect("Query to be successful") {
      return Ok(false);
    }

    // Hash pass
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = self.hasher.hash_password(pass.as_bytes(), &salt)?.to_string();

    // Create User
    let user = User::create_user(&self.connection_pool, &username, &password_hash)?;

    // Save cookie
    if save_cookie {
      let auth: Auth = (&user).into();
      self.cookie.write(&auth).expect("Write to be working, however not relevant for app to work...");
    }
    Ok(true)
  }

  // TODO: Delete Account, Change Password (this may result in changed gen-passwords -> found solution for this)
}
