use diesel::{Connection, PgConnection};

pub fn new_postgres_adapter(url: String) -> PgConnection {
    PgConnection::establish(&url)
        .unwrap_or_else(|_| panic!("Error connecting to postgres database: {}", url))
}
