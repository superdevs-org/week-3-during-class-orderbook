use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer};

use crate::{orderbook::Orderbook, routes::{create_order, delete_order, get_depth}};

pub mod routes;
pub mod inputs;
pub mod outputs;
pub mod orderbook;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let orderbook = Arc::new(Mutex::new(Orderbook::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(orderbook.clone())
            .service(create_order)
            .service(delete_order)
            .service(get_depth)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

