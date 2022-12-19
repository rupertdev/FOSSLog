use std::string;

use diesel::prelude::*;

#[derive(Queryable)]
pub struct QSO {
    pub id: i32,
    pub op_callsign: String,
    pub call: String,
    pub qso_date: i32,
    pub time_on: i32,
    pub band: String,
    pub mode: String,
}