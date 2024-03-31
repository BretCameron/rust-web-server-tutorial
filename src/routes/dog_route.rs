use crate::{
    models::dog_model::{Dog, DogRequest},
    services::db::Database,
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/dog")]
pub async fn create_dog(db: Data<Database>, request: Json<DogRequest>) -> HttpResponse {
    match db
        .create_dog(
            Dog::try_from(DogRequest {
                owner: request.owner.clone(),
                name: request.name.clone(),
                age: request.age.clone(),
                breed: request.breed.clone(),
            })
            .expect("Error converting DogRequest to Dog."),
        )
        .await
    {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
