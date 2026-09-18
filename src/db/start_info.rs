use crate::db::traits::{Create, Delete, Id, ReadByID, Update};
use rusqlite::{Connection, params};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessStartInfo {
    pub process_start_info_id: Option<i64>,
    pub name: String,
    pub path: String,
    pub args: String,
    pub cwd: String,
}

impl Id for ProcessStartInfo {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.process_start_info_id.unwrap_or(i64::MAX)
    }

    fn set_id(&mut self, id: Self::Id) {
        self.process_start_info_id = Some(id);
    }

    fn clear_id(&mut self) {
        self.process_start_info_id = None;
    }
}

pub struct ProcessStartInfoRepository;

impl Create<ProcessStartInfo> for ProcessStartInfoRepository {
    fn create(conn: &Connection, model: ProcessStartInfo) -> crate::Result<ProcessStartInfo>
    where
        Self: Sized,
    {
        let sql = r#"
        INSERT INTO process_start_info (path, args, cwd)
        VALUES (?, ?, ?)
        RETURNING process_start_info_id
"#;
        let mut model = model;
        let mut stmt = conn.prepare(sql)?;
        let id: i64 = stmt.query_row([&model.path, &model.args, &model.cwd], |row| row.get(0))?;
        model.process_start_info_id = Some(id);
        Ok(model)
    }
}

impl ReadByID<ProcessStartInfo> for ProcessStartInfoRepository {
    fn read_by_id(
        conn: &Connection,
        id: <ProcessStartInfo as Id>::Id,
    ) -> crate::Result<ProcessStartInfo> {
        let sql = r#"
        SELECT process_start_info_id, name, path, args, cwd
        FROM process_start_info
        WHERE process_start_info_id = ?
"#;
        let mut stmt = conn.prepare(sql)?;
        let row = stmt.query_row([id], |row| {
            Ok(ProcessStartInfo {
                process_start_info_id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                args: row.get(3)?,
                cwd: row.get(4)?,
            })
        })?;
        Ok(row)
    }
}

impl Update<ProcessStartInfo> for ProcessStartInfoRepository {
    fn update(conn: &Connection, dbo: &ProcessStartInfo) -> crate::Result<()> {
        if dbo.process_start_info_id.is_none() {
            return Err(crate::Error::UpdateDboHasNoId);
        }
        let sql = r#"
UPDATE process_start_info
SET name = ?, path = ?, args = ?, cwd = ?
WHERE process_start_info_id = ?
"#;
        let id = dbo.process_start_info_id.unwrap();
        let mut stmt = conn.prepare(sql)?;
        stmt.execute(params![&dbo.name, &dbo.path, &dbo.args, &dbo.cwd, id])?;
        Ok(())
    }
}

impl Delete<ProcessStartInfo> for ProcessStartInfoRepository {
    fn delete(
        conn: &Connection,
        id: <ProcessStartInfo as Id>::Id,
    ) -> crate::Result<ProcessStartInfo> {
        let mut to_delete = ProcessStartInfoRepository::read_by_id(conn, id)?;
        let sql = r#"
        DELETE FROM process_start_info
        WHERE process_start_info_id = ?
"#;
        let mut stmt = conn.prepare(sql)?;
        stmt.execute([id])?;
        to_delete.clear_id();
        Ok(to_delete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        crate::db::init_migration(&conn).expect("Failed to initialize database migrations");
        conn
    }

    #[test]
    fn test_create() {
        let conn = setup_test_db();
        let model = ProcessStartInfo {
            process_start_info_id: None,
            name: "python3".to_string(),
            path: "/usr/bin/python3".to_string(),
            args: "-m http.server 8080".to_string(),
            cwd: "/var/www".to_string(),
        };

        let created = ProcessStartInfoRepository::create(&conn, model)
            .expect("Failed to create process start info");

        assert_eq!(created.process_start_info_id, Some(1));
        assert_eq!(created.name, "python3");
        assert_eq!(created.path, "/usr/bin/python3");
        assert_eq!(created.args, "-m http.server 8080");
        assert_eq!(created.cwd, "/var/www");
    }

    #[test]
    fn test_read_by_id() {
        let conn = setup_test_db();
        let model = ProcessStartInfo {
            process_start_info_id: None,
            name: "rustc".to_string(),
            path: "/usr/bin/rustc".to_string(),
            args: "--version".to_string(),
            cwd: "/home/user".to_string(),
        };

        let created = ProcessStartInfoRepository::create(&conn, model)
            .expect("Failed to create process start info");
        let id = created.id();

        let read = ProcessStartInfoRepository::read_by_id(&conn, id)
            .expect("Failed to read process start info by id");

        assert_eq!(read, created);

        let non_existent = ProcessStartInfoRepository::read_by_id(&conn, 9999);
        assert!(non_existent.is_err());
    }

    #[test]
    fn test_update() {
        let conn = setup_test_db();
        let model = ProcessStartInfo {
            process_start_info_id: None,
            name: "node".to_string(),
            path: "/usr/bin/node".to_string(),
            args: "index.js".to_string(),
            cwd: "/app".to_string(),
        };

        let mut created = ProcessStartInfoRepository::create(&conn, model)
            .expect("Failed to create process start info");

        created.path = "/usr/local/bin/node".to_string();
        created.args = "server.js --port 3000".to_string();
        created.cwd = "/srv/app".to_string();

        ProcessStartInfoRepository::update(&conn, &created)
            .expect("Failed to update process start info");

        let read = ProcessStartInfoRepository::read_by_id(&conn, created.id())
            .expect("Failed to read updated process start info");

        assert_eq!(read, created);

        let without_id = ProcessStartInfo {
            process_start_info_id: None,
            name: "test".to_string(),
            path: "/usr/bin/test".to_string(),
            args: "".to_string(),
            cwd: "/".to_string(),
        };
        let err = ProcessStartInfoRepository::update(&conn, &without_id);
        assert!(matches!(err, Err(crate::Error::UpdateDboHasNoId)));
    }

    #[test]
    fn test_delete() {
        let conn = setup_test_db();
        let model = ProcessStartInfo {
            process_start_info_id: None,
            name: "bash".to_string(),
            path: "/usr/bin/bash".to_string(),
            args: "-c 'echo hello'".to_string(),
            cwd: "/tmp".to_string(),
        };

        let created = ProcessStartInfoRepository::create(&conn, model)
            .expect("Failed to create process start info");
        let id = created.id();

        let deleted = ProcessStartInfoRepository::delete(&conn, id)
            .expect("Failed to delete process start info");

        assert_eq!(deleted.process_start_info_id, None);
        assert_eq!(deleted.path, "/usr/bin/bash");
        assert_eq!(deleted.args, "-c 'echo hello'");
        assert_eq!(deleted.cwd, "/tmp");

        let read_after_delete = ProcessStartInfoRepository::read_by_id(&conn, id);
        assert!(read_after_delete.is_err());

        let delete_non_existent = ProcessStartInfoRepository::delete(&conn, 9999);
        assert!(delete_non_existent.is_err());
    }
}
