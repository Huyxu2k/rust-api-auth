use  crate::models::user::LoginInfoDTO;
use jsonwebtoken::{Header,EncodingKey};
use chrono::Utc;
use serde::{Deserialize,Serialize};


pub static KEY: &[u8]="271111".as_bytes();
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct  UserToken{
    pub iat: i64,
    pub exp: i64,
    pub user: String,
    pub login_session: String,
}
impl UserToken {
    pub fn generate_token(login:LoginInfoDTO)->String{
        let now=Utc::now().timestamp_nanos()/1_000_000_000;

        let payload=UserToken{
            iat:now,
            exp:now +ONE_WEEK,
            user:login.username.clone(),
            login_session:login.login_session.clone(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&KEY),
        ).unwrap()
    }
}