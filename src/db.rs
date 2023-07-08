use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

use crate::models::DataModel;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn save_data(pool: &DbPool, data: DataModel) -> QueryResult<usize> {
    use crate::schema::data::dsl::*;

    let new_data = NewData {
        name: data.name,
        value: data.value,
    };

    let conn: &SqliteConnection = &pool.get().unwrap();

    diesel::insert_into(data)
        .values(&new_data)
        .execute(conn)
}

pub fn get_data(pool: &DbPool) -> QueryResult<Vec<DataModel>> {
    use crate::schema::data::dsl::*;

    let conn: &SqliteConnection = &pool.get().unwrap();

    data.load::<DataModel>(conn)
}