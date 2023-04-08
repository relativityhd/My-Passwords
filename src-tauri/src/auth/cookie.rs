use std::{fs, str, path::PathBuf, convert::{TryFrom}, fmt::Display};
use serde::{Serialize, Deserialize};
use directories::ProjectDirs;
use chacha20poly1305::{
  aead::{Aead, AeadCore},
  XChaCha20Poly1305, XNonce
};
use rand_core::OsRng;

use super::error::CookieError;
use super::types::Cipher;

pub struct Cookie {
  cipher: Cipher,
  config_dir: PathBuf,
  nonce_path: PathBuf
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
  pub username: String,
  pub pass: String
}

impl Display for Auth {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let crypted_pass: String;
    if self.pass.len() < 3 {
      crypted_pass = "***".to_string();
    } else {
      let pass_first_chars: String = self.pass.chars().take(3).collect();
      let pass_last_char = self.pass.chars().last().unwrap_or('?');
      crypted_pass = format!("{}***{}", pass_first_chars, pass_last_char);
    }
    write!(f, "Cookie(username: {}, pass: {}*** ({} chars))", self.username, crypted_pass, self.pass.len())
  }
}

impl TryFrom<&Cookie> for Auth {
  type Error = CookieError;

  fn try_from(cookie: &Cookie) -> Result<Self, Self::Error> {
    cookie.read()?.ok_or(CookieError::Unpopulated)
  }
}

impl Cookie {
  pub fn new(cipher: Cipher) -> Cookie {
    let config_dir = ProjectDirs::from("org", "tobiashoelzer", "my-passwords")
      .expect("Project Dir should exist on normal machines...")
      .config_dir()
      .to_owned();

    let mut nonce_path = config_dir.clone();
    nonce_path.push("nonce");

    Cookie {cipher, config_dir, nonce_path}
  }

  pub fn read(&self) -> Result<Option<Auth>, CookieError> {
    /**
     * - Read nonce -> Used as path for cookie + as cryptic nonce for decryption
     * - Read encrypted cookie
     * - Decrypt cookie
     * - Parse Cookie from JSON
     * TODO: Replace JSON with efficient binary format
     */
    if !self.config_dir.exists() { return Ok(None); }

    // Read nonce
    if !self.nonce_path.exists() { return Ok(None); }
    let secret = fs::read_to_string(&self.nonce_path).map_err(|e| CookieError::IO(e, "reading nonce-file", self.nonce_path.clone()))?;

    // Read cookie
    let mut cookie_path = self.config_dir.clone();
    cookie_path.push(&secret);
    if !cookie_path.exists() { return Ok(None); }
    let cookie_encrypted = fs::read_to_string(&cookie_path).map_err(|e| CookieError::IO(e, "reading cookie-file", cookie_path))?;

    // Decrypt cookie
    let nonce = XNonce::from_slice(secret.as_bytes());
    let cookie_decrypted = self.cipher.decrypt(nonce, cookie_encrypted.as_ref()).map_err(|e| CookieError::Decrypt(e, nonce.to_owned(), cookie_encrypted))?;

    let cookie_serialized = str::from_utf8(&cookie_decrypted).map_err(|e| CookieError::Unrepresentable(e, cookie_decrypted.clone()))?;

    // Parse cookie
    let auth: Auth = serde_json::from_str(cookie_serialized).map_err(|e| CookieError::Unparsable(e, cookie_serialized.to_string()))?;

    Ok(Some(auth))
  }


  pub fn write(&self, auth: &Auth) -> Result<bool, CookieError> {
    /**
     * - Create new nonce
     * - Parse Cookie to JSON
     * - Encrypt Cookie
     * - Replace old nonce if exists, else just write it
     * - Replace encrypted Cookie if exists, else just write it
     * TODO: Replace JSON with efficient binary format
     */
    if !self.config_dir.exists() {
      fs::create_dir_all(&self.config_dir).map_err(|e| CookieError::IO(e, "creating cookie-directory", self.config_dir.clone()))?;
    }

    // Create new nonce
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

    // Parse Cookie to JSON
    let cookie_serialized = serde_json::to_string(&auth).map_err(|e| CookieError::Unparsable(e, auth.to_string()))?;

    // Encrypt Cookie
    let cookie_decrypted = str::as_bytes(&cookie_serialized);
    let cookie_encrypted = self.cipher.encrypt(&nonce, cookie_decrypted).map_err(|e| CookieError::Encrypt(e, nonce, cookie_serialized))?;

    // Write nonce
    fs::write(&self.nonce_path, nonce).map_err(|e| CookieError::IO(e, "writing nonce-file", self.nonce_path.clone()))?;

    // Write Cookie
    let secret = str::from_utf8(&nonce).map_err(|e| CookieError::Unrepresentable(e, nonce.to_vec()))?;
    let mut cookie_path = self.config_dir.clone();
    cookie_path.push(&secret);
    fs::write(&cookie_path, cookie_encrypted).map_err(|e| CookieError::IO(e, "writing cookie-file", cookie_path))?;

    Ok(true)
  }

  pub fn delete(&self) -> Result<(), CookieError> {
    if !self.config_dir.exists() { return Ok(()); }

    // Read & delete nonce
    if !self.nonce_path.exists() { return Ok(()); }
    let secret = fs::read_to_string(&self.nonce_path).map_err(|e| CookieError::IO(e, "reading nonce-file", self.nonce_path.clone()))?;

    fs::remove_file(&self.nonce_path).map_err(|e| CookieError::IO(e, "deleting nonce-file", self.nonce_path.clone()))?;

    // Get cookie_path
    let mut cookie_path = self.config_dir.clone();
    cookie_path.push(&secret);
    if !cookie_path.exists() { return Ok(()); }

    // Delete cookie
    fs::remove_file(&cookie_path).map_err(|e| CookieError::IO(e, "deleting cookie-file", cookie_path))?;

    Ok(())
  }
}
