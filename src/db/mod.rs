use rusqlite::Connection;
use tracing::debug;

mod migrations;
pub mod start_info;
pub mod traits;

pub fn connect<P>(path: P) -> crate::Result<Connection>
where
    P: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    debug!("Connecting to database: {:?}", path);
    Ok(Connection::open(path)?)
}

pub fn init<P>(path: P) -> crate::Result<()>
where
    P: AsRef<std::path::Path>,
{
    let path = path.as_ref();

    debug!("Initializing database: {:?}", path);
    let conn = Connection::open(path)?;

    init_migration(&conn)?;
    Ok(())
}

fn init_migration(conn: &Connection) -> crate::Result<()> {
    debug!("Initializing migration");
    init_version(conn)?;
    migrations::init(conn)?;
    Ok(())
}

fn table_exists(conn: &Connection, table_name: &str) -> crate::Result<bool> {
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?",
        [table_name],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

fn init_version(conn: &Connection) -> crate::Result<()> {
    if !table_exists(conn, "version")? {
        debug!("Creating a version table");
        conn.execute(
            r#"
CREATE TABLE version (version INTEGER NOT NULL)"#,
            [],
        )?;
        conn.execute("INSERT INTO version (version) VALUES (0)", [])?;
    }
    Ok(())
}

fn version(conn: &Connection) -> crate::Result<i64> {
    let version: i64 = conn.query_row("SELECT version FROM version", [], |row| row.get(0))?;
    Ok(version)
}

fn set_version(conn: &Connection, version: i32) -> crate::Result<()> {
    conn.execute("UPDATE version SET version = ?", [version])?;
    Ok(())
}
