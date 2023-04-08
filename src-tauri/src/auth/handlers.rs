use tauri::{AppHandle, Runtime};

use super::types::*;

#[tauri::command]
// this will be accessible with `invoke('plugin:session|check')`.
pub fn check<R: Runtime>(_app: AppHandle<R>, connection_pool: ConnectionPool) -> Result<bool, String> {
  return Ok(true);
}

#[tauri::command]
// this will be accessible with `invoke('plugin:session|signup')`.
pub fn signup<R: Runtime>(_app: AppHandle<R>, connection_pool: ConnectionPool, uname: &str, raw_pass: &str) -> Result<bool, String> {
  return Ok(true);

}