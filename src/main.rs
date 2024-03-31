mod models;
mod routes;
mod services;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use routes::{
    booking_route::{cancel_booking, create_booking, get_bookings},
    dog_route::create_dog,
    owner_route::create_owner,
};
use services::db::Database;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Medium!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(create_owner)
            .service(create_dog)
            .service(create_booking)
            .service(get_bookings)
            .service(cancel_booking)
    })
    .bind(("127.0.0.1", 5001))?
    .run()
    .await
}
