use actix_web::{web, Error, HttpResponse, Responder};
use serde::Serialize;

/// A simple struct that we’ll return as JSON for GET /api/v1/items.
#[derive(Serialize)]
struct Item {
    id: u32,
    name: String,
}

/// Handler for GET /api/v1/items
pub async fn list_items() -> Result<impl Responder, Error> {
    let items = vec![
        Item { id: 1, name: "Maxwell".into() },
        Item { id: 2, name: "Bingus".into() },
        Item { id: 3, name: "Floppa".into() },
    ];
    Ok(HttpResponse::Ok().json(items))
}

/// Returns an Actix `Scope` that you can register into `App::new()`.
pub fn api_scope() -> actix_web::Scope {
    web::scope("/api/v1")
        .route("/items", web::get().to(list_items))
}
