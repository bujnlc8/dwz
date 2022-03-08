use super::schema::dwz;
use diesel::Queryable;
use std::fmt::Display;

#[derive(Queryable)]
pub struct Dwz {
    pub id: u32,
    pub short_url: String,
    pub redirect_url: String,
    pub valid_time: chrono::NaiveDateTime,
    pub create_time: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "dwz"]
pub struct NewDwz<'a> {
    pub short_url: &'a str,
    pub redirect_url: &'a str,
    pub valid_time: chrono::NaiveDateTime,
    pub create_time: chrono::NaiveDateTime,
}

pub struct CommonError {
    pub message: String,
}

impl Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
