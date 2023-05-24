use actix_web::{web,HttpResponse};
use chrono::Utc;
use sqlx::{MySqlPool};
use serde::{Deserialize,Serialize};


#[derive(Deserialize,Serialize)]
pub struct Users_Test{
    pub email:String,
    pub  name:String,
}

pub async fn subscribe(form: web::Form<Users_Test>,pool: web::Data<MySqlPool>,) -> HttpResponse {
    let sqlstring = &format!("INSERT INTO Users (email, name, subscribed_at) VALUES ({0}, {1}, {2})",form.email,form.name,Utc::now());
    match sqlx::query(&sqlstring).execute(pool.get_ref()).await{
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    } 
    
}
