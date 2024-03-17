use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use polodb_core::{Database, Collection};
use polodb_core::bson::{Document, doc};
use std::sync::Arc;

// Define your function to find a game for a user
fn find_game_for_user(collection: &Collection<Document>, game_name: &str) -> Option<Document> {
    let query = doc! {
        "name": game_name,
    };

    collection.find_one(query).unwrap()
}

// Define a handler function for your endpoint
async fn find_game_handler(data: web::Data<Arc<Collection<Document>>>, game_name: web::Path<String>) -> impl Responder {
    // Call your function to find the game
    match find_game_for_user(&data.as_ref(), &game_name) {
        Some(result) => HttpResponse::Ok().json(result),
        None => HttpResponse::NotFound().body("Game not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to your database
    let db = Database::open_file("/workspaces/Videogame-SQL-Search---Rust-Actix-Microservice-with-AWS-RDS/base/test-polo.db").unwrap();
    let collection = db.collection::<Document>("games");

    // Wrap the collection in an Arc to share across threads
    let data = web::Data::new(Arc::new(collection));

    // Start your Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone()) // Pass your shared collection data to each request handler
            .route("/find_game/{game_name}", web::get().to(find_game_handler)) // Define your endpoint
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
