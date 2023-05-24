use chrono::{Utc,NaiveDateTime};
use sqlx::{MySqlPool, Error,FromRow,Row, mysql::{MySqlRow, MySqlQueryResult}};
use uuid::Uuid;
use serde::{Deserialize,Serialize};
use crate::models::user::User;

#[derive(Debug,Clone)]
pub struct LoginHistory{
    pub id:i32,
    pub user_id:i32,
    pub login_timestamp:NaiveDateTime
}
#[derive(Debug,Clone)]
pub struct LoginHistoryDTO{
    pub user_id:i32,
    pub login_timestamp:NaiveDateTime
}
impl LoginHistory{
      pub async fn create(use_name:String,pool:MySqlPool)->Option<LoginHistoryDTO>{
         if let Ok(user)=User::find_user_by_username(use_name, pool).await{
            let now=Utc::now();
            Some(LoginHistoryDTO{
                user_id:user.id,
                login_timestamp:now.naive_utc(),//Utc::now().naive_utc()
            })
         }
         else {
             None
         }
      }
      pub async fn save_login_history(login_history:LoginHistoryDTO,pool:MySqlPool)->bool{
          let query_str=format!("insert into login_history(user_id,login_timestamp) values ({},'{}')",login_history.user_id,login_history.login_timestamp);
 
          let result=sqlx::query(&query_str)
                                                       .execute(&pool.clone())
                                                       .await
                                                       .is_ok();
         if result {
            true
         }
         else {
             false
         }
          
      }
}
