use actix_web::{
    http::{header::HeaderValue, StatusCode},
};
use serde_json::json;
use sqlx::{MySqlPool};
use serde::{Deserialize,Serialize};
use crate::models::{ 
    user::{LoginDTO,NewUser,User},
    user_token::UserToken,
};
use  crate::utils::{
    error::ServiceError,
    token_utils,
};

#[derive(Deserialize,Serialize)]
pub struct  TokenBodyResponse{
    pub token:String,
    pub token_type:String,
}

pub async fn signup(user:NewUser,pool:MySqlPool)->Result<String,ServiceError>{

     match User::signup(user,pool).await {
         Ok(message)=>Ok(message),
         Err(message)=>Err(ServiceError::new(StatusCode::BAD_REQUEST, message)),
     }
}
pub async fn login(user:LoginDTO,pool:MySqlPool)->Result<TokenBodyResponse,ServiceError>{
   match User::login(user, pool).await {
       Some(logger_user)=>{
        let result =serde_json::from_value(json!({ "token": UserToken::generate_token(logger_user.clone()), "token_type": "bearer" }),);
        match  result {
          Ok(token_res)=>{
            if logger_user.login_session.is_empty(){
                 Err(ServiceError::new(
                                       StatusCode::UNAUTHORIZED, 
                                       "Login Failed".to_string(),
                                      )
                    )
            }
            else {
                Ok(token_res)
            }
          },
          Err(_)=>{
            Err(
                ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string()
                )
            )
          },
        }
       },
       None=>Err(
        ServiceError::new(
            StatusCode::UNAUTHORIZED,
            "user not found".to_string()
        )
       ),
   }
}
//TODO error
pub async fn logout(authen_header:HeaderValue,pool:MySqlPool)->Result<(),ServiceError>{
    if let Ok(authen_str)=authen_header.to_str(){
        if authen_str.starts_with("Bearer"){
            let token=authen_str[6..authen_str.len()].trim();
            if let Ok(token_data)=token_utils::decode_token(token.to_string()){
                if let Ok(user_name)=token_utils::verify_token(&token_data, pool.clone()).await{
                    if let Ok(user)=User::find_user_by_username(user_name, pool.clone()).await{
                        User::logout(user.id,pool).await;
                        return Ok(());
                    }
                }
            }
        }
    }
    Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error while processing token".to_string()))
}
pub async fn getuserbyusername(authen_header:HeaderValue,pool:MySqlPool)->Result<User,ServiceError>{
    if let Ok(authen_str)=authen_header.to_str(){
        if authen_str.starts_with("Bearer"){
            let token=authen_str[6..authen_str.len()].trim();
            if let Ok(token_data)=token_utils::decode_token(token.to_string()){
                if let Ok(user_name)=token_utils::verify_token(&token_data, pool.clone()).await{
                    if let Ok(user)=User::find_user_by_username(user_name, pool.clone()).await{
                        return Ok(user);
                    }
                }
            }
        }
    }
    Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error while processing token".to_string()))
}