use diesel::{Connection, PgConnection};

pub fn new_postgres_adapter(database_url: String) -> PgConnection {
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to postgres database: {}", database_url))
}
