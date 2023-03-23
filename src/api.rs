use actix_web::{get, web, Responder, Result};

use crate::types::TodoItem;

#[get("/todos")]
pub async fn get_todos() -> Result<impl Responder> {
    let mut items: Vec<TodoItem> = vec![];
    for i in 1..=3 {
        items.push(TodoItem {
            id: format!("{}", i),
            first_name: "Alistair".into(),
            last_name: "Foggin".into(),
            description: format!("Todo item number {}", i),
            time_created: "".into(),
        })
    }
    Ok(web::Json(items))
}

