use std::sync::{Arc, Mutex};

use actix_web::{delete, get, post, web::{self, Json, Data}, HttpResponse,Responder};

use crate::{inputs::{CreateOrderInput, DeleteOrder}, orderbook::{self, Orderbook}, outputs::{CreateOrderResponse, DeleteOrderResponse, Depth}};

#[post("/order")]
pub async fn create_order(body: Json<CreateOrderInput>, orderbook: Data<Arc<Mutex<Orderbook> > >) -> impl Responder {
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;
    
    // maintain orderbook logic
    let mut orderbook = orderbook.lock().unwrap();
    orderbook.create_order(price, quantity, user_id, side);

    return HttpResponse::Ok().json(CreateOrderResponse {
        order_id: String::from("ads")
    });
}

#[delete("/order")]
pub async fn delete_order(Json(body): Json<DeleteOrder>, orderbook: Data<Orderbook>) -> impl Responder {
    let order_id = body.order_id;
    HttpResponse::Ok().json(DeleteOrderResponse {
        filled_qty: 0,
        average_price: 100
    })
}

#[get("/depth")]
pub async fn get_depth(orderbook: Data<Orderbook>) -> impl Responder {
    let depth = orderbook.get_depth();
    HttpResponse::Ok().json(Depth {
        bids: vec![],
        asks: vec![],
        lastUpdateId: String::from("adsa")
    })
}