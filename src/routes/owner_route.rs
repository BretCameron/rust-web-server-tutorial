use crate::{
    models::owner_model::{Owner, OwnerRequest},
    services::db::Database,
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/owner")]
pub async fn create_owner(db: Data<Database>, request: Json<OwnerRequest>) -> HttpResponse {
    match db
        .create_owner(
            Owner::try_from(OwnerRequest {
                name: request.name.clone(),
                email: request.email.clone(),
                phone: request.phone.clone(),
                address: request.address.clone(),
            })
            .expect("Error converting OwnerRequest to Owner."),
        )
        .await
    {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
