use actix_web::web::{Json,  Path};
use actix_web::{get, web};
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::APPLICATION_JSON;
use crate::response::Response;

use crate::flux::read_flux;
use crate::flux::write_flux;

pub type Temps = Response<Temp>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Temp {
    pub room: String,
    pub temp : String
}

impl Temp {
    pub fn new(room: String, temp: String) -> Self {
        Self {
            room:  room,
            temp: temp,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TempRequest {
    pub room: String,
    pub temp : String
}

impl TempRequest {
    pub fn to_temp(&self) -> Option<Temp> {
    
        let room = &self.room;
        let temp = &self.temp;
        
        if!room.is_empty() && !temp.is_empty()
        {
           return Some(Temp::new(room.clone(),temp.clone()));
        }
        else 
        {
            return None;
        }
    }
}

#[get("/temp")]
pub async fn list() -> HttpResponse {
    

    let tweets = Temps { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets)
}


/// create a temp `/temp`
#[post("/temp")]
pub async fn create(temp_req: web::Query<TempRequest> ) -> HttpResponse {

    //let resp: String = format!("new temp:: room={}  temp={}", temp_req.room, temp_req.temp);
    
    let temp: f64 = temp_req.temp.parse().unwrap();
    let room: String =temp_req.room.clone(); 
    let response1 = write_flux(room, temp).await;
    let r1 = response1.unwrap();

    //HttpResponse::Ok().body(format!("{:?}", resp));

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(temp_req.to_temp())
}

/// find a temp by its id `/temp/{id}`
#[get("/temp/{id}")]
pub async fn get(path: Path<(String,)>) -> HttpResponse {
    // TODO find temp by ID and return it
    let found_temp: Option<Temp> = None;

    match found_temp {
        Some(temp) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(temp),
        None => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

/// delete a temp by its id `/temp/{id}`
#[delete("/temp/{id}")]
pub async fn delete(path: Path<(String,)>) -> HttpResponse {
    // TODO delete temp by ID
    // in any case return status 204

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}