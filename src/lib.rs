#![allow(unused)]

pub mod db;
mod error;

pub use error::{Error, Result};

pub fn default_db_path() -> Result<std::path::PathBuf> {
    match std::env::var("PROC_MAN_DIR") {
        Ok(path) => Ok(std::path::PathBuf::from(path).join("proc_man.sqlite")),
        Err(_) => Ok(dirs::config_local_dir()
            .ok_or(Error::ConfigLocalDirNotFound)?
            .join("proc_man")
            .join("proc_man.db")),
    }
}
