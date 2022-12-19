CREATE TABLE qsos (
    id INTEGER PRIMARY KEY NOT NULL,
    op_callsign TEXT NOT NULL,
    call TEXT NOT NULL,
    qso_date INTEGER NOT NULL,
    time_on INTEGER NOT NULL,
    band TEXT NOT NULL,
    mode TEXT NOT NULL
);