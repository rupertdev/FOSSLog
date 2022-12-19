use self::models::*;
use diesel::prelude::*;
use foss_log::*;

fn main() {
    use self::schema::qsos::dsl::*;

    let connection = &mut establish_connection();
    let results = qsos
        .limit(5)
        .load::<QSO>(connection)
        .expect("Error loading qsos");

    println!("Displaying {} posts", results.len());
    for qso in results {
        println!("{}", qso.op_callsign);
        println!("-----------\n");
        println!("{}", qso.qso_date);
    }
}