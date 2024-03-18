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

// Define a handler function for the root URL
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the game search app! \n\n\n 
    
    You can search for the sales information regarding any game that you want. The search is case sensitive. \n
    Use the functionality by adding this after the last slash at the link (or add a slash if you don't see any) \n
    
    /find_game/{game_name} \n\n

    Substitute {game_name} for any game. Try 'Tetris' to get you started!!!! \n\n\n

    Thanks for trying this app out!!!!")
}

// Define a handler function for your endpoint
async fn find_game_handler(data: web::Data<Arc<Collection<Document>>>, game_name: web::Path<String>) -> impl Responder {
    // Call your function to find the game
    match find_game_for_user(&data.as_ref(), &game_name) {
        Some(result) => {
            // Convert the JSON result to a newline-separated string
            let result_string = serde_json::to_string_pretty(&result).unwrap_or_else(|_| String::from("Failed to serialize result"));
            let result_string_with_newlines = result_string.replace(", ", ",\n");

            // Return the response with newline-separated content
            HttpResponse::Ok().body(result_string_with_newlines)
        },
        None => HttpResponse::NotFound().body("Game not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to your database
    let db = Database::open_file("vgtest-polo.db").unwrap();
    let collection = db.collection::<Document>("games");

    // Wrap the collection in an Arc to share across threads
    let data = web::Data::new(Arc::new(collection));

    // Start your Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone()) // Pass your shared collection data to each request handler
            .route("/", web::get().to(index)) // Define root URL handler
            .route("/find_game/{game_name}", web::get().to(find_game_handler)) // Define your endpoint
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

