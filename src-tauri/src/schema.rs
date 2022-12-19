// @generated automatically by Diesel CLI.

diesel::table! {
    qsos (id) {
        id -> Integer,
        op_callsign -> Text,
        call -> Text,
        qso_date -> Integer,
        time_on -> Integer,
        band -> Text,
        mode -> Text,
    }
}
