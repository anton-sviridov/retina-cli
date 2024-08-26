use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::detections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Detection {
    pub id: i32,
    pub image: String,
    pub date: String,
    pub structure: String
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::detections)]
pub struct NewDetection<'a> {
    pub image: &'a str,
    pub date: &'a str,
    pub structure: String
}