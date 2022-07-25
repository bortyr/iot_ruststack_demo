#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use rocket_contrib::json::Json;
use rocket_contrib::database;
use diesel::prelude::*;
use models::Measurement;
use models::NewMeasurement;
use crate::schema::measurements;

pub mod models;
pub mod schema;

#[database("postgres")]
struct DbConn(PgConnection);

// Get measurements from the PostgreSQL
#[get("/getm.json")]
fn get_measurements(conn: DbConn) -> Json<Vec<Measurement>> {
    let readings = measurements::table
        .order(measurements::columns::id.desc())
        .load::<Measurement>(&*conn)
        .unwrap();
    Json(readings)
}

// Receive JSON and save it to PostgreSQL
#[post("/add", format = "json", data = "<measurement>")]
fn new_measurement(conn: DbConn, measurement: Json<NewMeasurement>) -> Json<Measurement> {
    let result = diesel::insert_into(measurements::table)
        .values(&*measurement)
        .get_result(&*conn)
        .unwrap();
    Json(result)
}

// Check health
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().attach(DbConn::fairing()).mount("/", routes![index, get_measurements, new_measurement]).launch();
}
