#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to get user config directory")]
    ConfigLocalDirNotFound,
    #[error("Database error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Update failed: dbo has no id")]
    UpdateDboHasNoId,
}

pub type Result<T> = std::result::Result<T, Error>;
