use actix_web::{App, HttpServer};
use sqlx::{postgres::PgConnection, Connection};

use crate::api::get_todos;

mod api;
mod db;
mod types;

#[derive(sqlx::FromRow, Debug)]
struct User {
    userid: String,
    firstname: String,
    lastname: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://postgres:password@localhost/test").await.unwrap();
    let mut conn = PgConnection::connect("postgres://postgres:password@localhost")
        .await
        .unwrap();

    let rows = sqlx::query_as::<_, User>("SELECT * FROM users").fetch_all(&mut conn).await.unwrap();
    for row in rows {
        println!("{:?}", row);
    }

    HttpServer::new(|| App::new().service(get_todos))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
