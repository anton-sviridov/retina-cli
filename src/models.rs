use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::detections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Detection {
    pub id: i32,
    pub image: String,
    pub description: String,
    pub date: String,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::detections)]
pub struct NewDetection<'a> {
    pub image: &'a str,
    pub description: &'a str,
    pub date: &'a str,
}