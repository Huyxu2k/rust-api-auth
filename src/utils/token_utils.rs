use jsonwebtoken::{DecodingKey, TokenData, Validation};
use sqlx::MySqlPool;
use  crate::{
    models::{
        user::User,
        user_token::{KEY,UserToken}},
};

pub fn decode_token(token:String)->jsonwebtoken::errors::Result<TokenData<UserToken>>{
    jsonwebtoken::decode::<UserToken>(
             &token,
             &DecodingKey::from_secret(&KEY),
             &Validation::default()
            )
}
pub async fn verify_token(token_data:&TokenData<UserToken>,pool:MySqlPool)->Result<String,String>{
    if User::is_valid_login_session(token_data.claims.clone(), &pool).await{
        Ok(token_data.claims.user.to_string())
    }else {
        Err("Invalid Token".to_string())
    }
}