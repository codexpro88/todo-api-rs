use actix_web::{get, web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TodoItem {
    id: String,
    first_name: String,
    last_name: String,
    description: String,
    time: String,
}

#[get("/todos")]
async fn get_todos() -> Result<impl Responder> {
    let mut items: Vec<TodoItem> = vec![];
    for i in 1..=3 {
        items.push(TodoItem {
            id: format!("{}", i),
            first_name: "Alistair".into(),
            last_name: "Foggin".into(),
            description: format!("Todo item number {}", i),
            time: "".into(),
        })
    }
    Ok(web::Json(items))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(get_todos))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
