use rusqlite::Connection;

pub trait Id {
    type Id;

    fn id(&self) -> Self::Id;
    fn set_id(&mut self, id: Self::Id);
    fn clear_id(&mut self);
}

pub trait Create<Dbo> {
    fn create(conn: &Connection, dbo: Dbo) -> crate::Result<Dbo>;
}

pub trait ReadByID<Dbo>
where
    Dbo: Id,
{
    fn read_by_id(conn: &Connection, id: Dbo::Id) -> crate::Result<Dbo>;
}
pub trait Update<Dbo> {
    fn update(conn: &Connection, dbo: &Dbo) -> crate::Result<()>;
}

pub trait Delete<Dbo>
where
    Dbo: Id,
{
    fn delete(conn: &Connection, id: Dbo::Id) -> crate::Result<Dbo>;
}

pub trait ListAll<Dbo> {
    fn list_all(conn: &Connection) -> crate::Result<Vec<Dbo>>;
}