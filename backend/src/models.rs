use serde_derive::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use crate::schema::measurements;

#[derive(Queryable,Serialize)]
pub struct Measurement {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "measurements"]
pub struct NewMeasurement<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
