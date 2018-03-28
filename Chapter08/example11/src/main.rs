#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_infer_schema;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("error connecting to {}", database_url))
}

mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

#[derive(Debug, Queryable)]
struct User {
    username: String,
    password: String,
    email: Option<String>,
}

fn main() {
    use schema::users::dsl::*;

    let connection = establish_connection();
    let all_users = users
        .load::<User>(&connection)
        .expect("error loading users");

    println!("{:?}", all_users);
}
