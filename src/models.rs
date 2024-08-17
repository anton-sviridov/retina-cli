use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::memories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Memory {
    pub id: i32,
    pub image: String,
    pub description: String,
    pub date: String,
}