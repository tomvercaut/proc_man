use crate::db;
use crate::db::set_version;
use lazy_static::lazy_static;
use rusqlite::Connection;
use std::collections::HashMap;
use tracing::debug;

pub type MigrationFn = fn(&Connection) -> crate::Result<()>;

lazy_static! {
    pub(crate) static ref MIGRATIONS: HashMap<i64, MigrationFn> = {
        let mut m = HashMap::new();
        m.insert(1, v1 as MigrationFn);
        m.insert(2, v2 as MigrationFn);
        m
    };
}

fn v1(conn: &Connection) -> crate::Result<()> {
    conn.execute(
        r#"CREATE TABLE process_start_info (
            process_start_info_id INTEGER PRIMARY KEY,
            path TEXT NOT NULL,
            args TEXT NOT NULL,
            cwd TEXT NOT NULL
        )"#,
        [],
    )?;
    set_version(conn, 1)?;
    Ok(())
}

fn v2(conn: &Connection) -> crate::Result<()> {
    conn.execute(
        r#"ALTER TABLE process_start_info ADD COLUMN name TEXT NOT NULL"#,
        [],
    )?;
    set_version(conn, 2)?;
    Ok(())
}

pub(crate) fn init(con: &Connection) -> crate::Result<()> {
    debug!("Initializing migrations");
    let mut vers = db::version(con)?;
    loop {
        let next_version = vers + 1;
        if let Some(migration) = MIGRATIONS.get(&next_version) {
            debug!("Running migration v{}", next_version);
            migration(con)?;
            vers = next_version;
        } else {
            break;
        }
    }
    Ok(())
}
