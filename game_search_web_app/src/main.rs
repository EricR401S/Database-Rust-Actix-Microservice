// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use polodb_core::{Database, Collection};
// use polodb_core::bson::{Document, doc};
// use std::sync::Arc;

// // Define your function to find a game for a user
// fn find_game_for_user(collection: &Collection<Document>, game_name: &str) -> Option<Document> {
//     let query = doc! {
//         "name": game_name,
//     };

//     collection.find_one(query).unwrap()
// }

// // Define a handler function for the root URL
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Welcome to the game search app! \n\n\n 
    
//     You can search for the sales information regarding any game that you want. The search is case sensitive. \n
//     Use the functionality by adding this after the last slash at the link (or add a slash if you don't see any) \n
    
//     /find_game/{game_name} \n\n

//     Substitue {game_name} for any game. Try 'Tetris' to get you started!!!! \n\n\n

//     Thanks for trying this app out!!!!")
// }

// // Define a handler function for your endpoint
// async fn find_game_handler(data: web::Data<Arc<Collection<Document>>>, game_name: web::Path<String>) -> impl Responder {
//     // Call your function to find the game
//     match find_game_for_user(&data.as_ref(), &game_name) {
//         Some(result) => HttpResponse::Ok().json(result),
//         None => HttpResponse::NotFound().body("Game not found"),
//     }
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     // Download the database file from the URL
//     let url = "https://vgexo.s3.us-west-1.amazonaws.com/vgtest-polo.db";
//     let database_file = reqwest::get(url)
//         .await
//         .expect("Failed to download database")
//         .bytes()
//         .await
//         .expect("Failed to read database");

//     // Save the downloaded database file
//     let path_to_db = "vgtest-polo.db";
//     std::fs::write(path_to_db, database_file)
//         .expect("Failed to write database file");

//     // Connect to your database
//     let db = Database::open_file(path_to_db).unwrap();
//     let collection = db.collection::<Document>("games");

//     // Wrap the collection in an Arc to share across threads
//     let data = web::Data::new(Arc::new(collection));

//     // Start your Actix web server
//     HttpServer::new(move || {
//         App::new()
//             .app_data(data.clone()) // Pass your shared collection data to each request handler
//             .route("/", web::get().to(index)) // Define root URL handler
//             .route("/find_game/{game_name}", web::get().to(find_game_handler)) // Define your endpoint
//     })
//     // .bind(""127.0.0.1:8080"")?
//     .bind("0.0.0.0:8080")?
//     .run()
//     .await
// }


use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use polodb_core::{Database, Collection};
use polodb_core::bson::{Document, doc};
use std::sync::Arc;
use std::fs;

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

// // Define a handler function for the root URL
// async fn index() -> impl Responder {
//     // Print all files in the working directory
//     let files = fs::read_dir(".").unwrap();
//     for file in files {
//         if let Ok(file) = file {
//             println!("Filename: {:?}", file.file_name());
//         }
//     }
    
//     HttpResponse::Ok().body("Welcome to the game search app! \n\n\n 
    
//     You can search for the sales information regarding any game that you want. The search is case sensitive. \n
//     Use the functionality by adding this after the last slash at the link (or add a slash if you don't see any) \n
    
//     /find_game/{game_name} \n\n

//     Substitue {game_name} for any game. Try 'Tetris' to get you started!!!! \n\n\n

//     Thanks for trying this app out!!!!")
// }

// Define a handler function for the root URL
async fn index() -> impl Responder {
    // Get filenames in the working directory
    let mut files_list = String::new();
    if let Ok(files) = fs::read_dir(".") {
        for file in files {
            if let Ok(file) = file {
                if let Some(file_name) = file.file_name().to_str() {
                    files_list.push_str(&format!("Filename: {:?}\n", file_name));
                }
            }
        }
    }

    let welcome_message = format!("Welcome to the game search app! \n\n\n 
    
    You can search for the sales information regarding any game that you want. The search is case sensitive. \n
    Use the functionality by adding this after the last slash at the link (or add a slash if you don't see any) \n
    
    /find_game/{{game_name}} \n\n

    Substitute {{game_name}} for any game. Try 'Tetris' to get you started!!!! \n\n\n

    Thanks for trying this app out!!!!\n\nFiles in the working directory:\n{}", files_list);

    HttpResponse::Ok().body(welcome_message)
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
    // .bind("127.0.0.1:8080")?
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
