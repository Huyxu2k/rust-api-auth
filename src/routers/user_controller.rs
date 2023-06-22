use crate::{
    models::{
        response::ResponseBody,
        user::{LoginDTO, NewUser},
    },
    services::user_service,
};
use actix_web::{web, HttpRequest, HttpResponse, Result, post};
use sqlx::{ MySqlPool};

// POST api/auth/signup
//#[post("/signup")]
pub async fn signup(
    user_dt: web::Json<NewUser>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    match user_service::signup(user_dt.0, pool.get_ref().to_owned()).await {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(message, ""))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/auth/login
pub async fn login(
    login_dt: web::Json<LoginDTO>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    match user_service::login(login_dt.0, pool.get_ref().to_owned()).await {
        Ok(token_res) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new("Login success".to_string(), token_res)))
        }
        Err(err) => Ok(err.response()),
    }
}

// POST api/auth/logout
pub async fn logout(req: HttpRequest, pool: web::Data<MySqlPool>) -> Result<HttpResponse> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        user_service::logout(auth_header.clone(), pool.get_ref().to_owned()).await;
        Ok(HttpResponse::Ok().json(ResponseBody::new("Logout Success".to_string(), "")))
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new("Token Missing".to_string(), "")))
    }
}

//GET api/auth/profile
pub async fn getuserbyusername(req: HttpRequest,pool: web::Data<MySqlPool>)->Result<HttpResponse>{
    if let Some(auth_header) = req.headers().get("Authorization") {
       let user= user_service::getuserbyusername(auth_header.clone(), pool.get_ref().to_owned()).await.ok();
        Ok(HttpResponse::Ok().json(ResponseBody::new("Success".to_string(), user)))
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new("Token Missing".to_string(), "")))
    }
}
