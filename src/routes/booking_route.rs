use crate::{
    models::booking_model::{Booking, BookingRequest},
    services::db::Database,
};
use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

#[get("/bookings")]
pub async fn get_bookings(db: Data<Database>) -> HttpResponse {
    match db.get_bookings().await {
        Ok(bookings) => HttpResponse::Ok().json(bookings),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/booking/{id}/cancel")]
pub async fn cancel_booking(db: Data<Database>, path: Path<(String,)>) -> HttpResponse {
    let id = path.into_inner().0;

    match db.cancel_booking(id.as_str()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/booking")]
pub async fn create_booking(db: Data<Database>, request: Json<BookingRequest>) -> HttpResponse {
    match db
        .create_booking(
            Booking::try_from(BookingRequest {
                owner: request.owner.clone(),
                start_time: request.start_time.clone(),
                duration_in_minutes: request.duration_in_minutes.clone(),
            })
            .expect("Error converting BookingRequest to Booking."),
        )
        .await
    {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
