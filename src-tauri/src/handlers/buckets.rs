use sea_orm::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BucketError {
    #[error("Database error from SeaORM: {0:?}")]
    Db(#[from] DbErr),
}

#[tauri::command]
#[specta::specta]
pub async fn create_bucket() -> Result<(), BucketError> {
    todo!()
}

#[tauri::command]
#[specta::specta]
pub async fn recolor_bucket() -> Result<(), BucketError> {
    todo!()
}

#[tauri::command]
#[specta::specta]
pub async fn rename_bucket() -> Result<(), BucketError> {
    todo!()
}

#[tauri::command]
#[specta::specta]
pub async fn delete_bucket() -> Result<(), BucketError> {
    todo!()
}
