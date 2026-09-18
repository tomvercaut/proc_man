use crate::db::traits::{Create, Delete, Id, ReadByID, Update};
use rusqlite::{Connection, params};

#[derive(Clone, Debug)]
pub struct ProcessStartInfo {
    pub process_start_info_id: Option<i64>,
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
        SELECT process_start_info_id, path, args, cwd
        FROM process_start_info
        WHERE process_start_info_id = ?
"#;
        let mut stmt = conn.prepare(sql)?;
        let row = stmt.query_row([id], |row| {
            Ok(ProcessStartInfo {
                process_start_info_id: row.get(0)?,
                path: row.get(1)?,
                args: row.get(2)?,
                cwd: row.get(3)?,
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
SET path = ?, args = ?, cwd = ?
WHERE process_start_info_id = ?
"#;
        let id = dbo.process_start_info_id.unwrap();
        let mut stmt = conn.prepare(sql)?;
        stmt.execute(params![&dbo.path, &dbo.args, &dbo.cwd, id])?;
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
