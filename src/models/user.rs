use std::result;
use chrono::{Utc,DateTime, NaiveDate };
use sqlx::{MySqlPool, Error,FromRow,Row, mysql::{MySqlRow}};
use uuid::Uuid;
use serde::{Deserialize,Serialize};
use bcrypt::{hash,verify,DEFAULT_COST};
use  crate::models::{
                user_token::{UserToken},
                login_history::LoginHistory,
           };



#[derive(Deserialize,Serialize,Debug,Clone,FromRow)]
pub struct User{
    pub id:i32,
    pub email:String,
    pub user_name:String,
    pub password:String,
    pub login_session:String
}
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct NewUser{
    pub email:String,
    pub user_name:String,
    pub password:String,
}
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct LoginDTO{
    pub email_or_user_name:String,
    pub password:String,
}
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct LoginInfoDTO {
    pub username: String,
    pub login_session: String,
}

impl User{
    //new user
    pub fn new_user(self)->User{
        User {
             id: self.id,
             email: self.email,
             user_name:self.user_name,
             password: self.password,
             login_session:self.login_session
            }
    }
    //signup
    pub async fn signup(user_o:NewUser,pool:MySqlPool)->Result<String,String>{
        let user_find=Self::find_user_by_username(user_o.user_name.clone(), pool.clone()).await;
            match user_find {
                Ok(user_find )=>{
                    Err(format!("User '{}' is already registered", &user_o.user_name))
                },
                Err(_) => {
                    let hash_pw=hash(user_o.password, DEFAULT_COST).unwrap();
                    let user=NewUser{
                        email:user_o.email,
                        user_name:user_o.user_name.clone(),
                        password:hash_pw,
                    };
                    let datein = Utc::now().naive_utc();
                    let query_str=format!("insert into users(email,username,password,subscribed_at) values ('{}','{}','{}','{}')",user.email,user.user_name,user.password,datein);
                    let result= sqlx::query(&query_str)//.bind()
                         .execute(&pool.clone())
                         .await;
                    //assert_eq!(result,None);
                    Ok("Signup successfully".to_string())
                }
            }
    }
    //login
    pub async fn login(login:LoginDTO,pool:MySqlPool)->Option<LoginInfoDTO>{
       let query_str1=format!("select * from users where username='{}' or email='{}'",login.email_or_user_name,login.email_or_user_name);
       let user=sqlx::query(&query_str1)
                                        .map(|row:MySqlRow|User{
                                            id:row.get("id"),
                                            email:row.get("email"),
                                            user_name:row.get("username"),
                                            password:row.get("password"),
                                            login_session:row.get("login_session")
                                        })
                                        .fetch_one(&pool.clone())
                                        .await
                                        .ok();
         match user {
            Some(user) => {
             if !user.password.is_empty() && verify(&login.password, &user.password).unwrap(){
                 if let Some(login_history)=LoginHistory::create(user.user_name.clone(), pool.clone()).await{
                    let result_save=LoginHistory::save_login_history(login_history, pool.clone()).await;
                    if !result_save {
                        return None;
                    }
                    let login_session_str = User::generate_login_session();
                    let result_update=User::update_login_session(user.user_name.clone(), login_session_str.clone(), pool).await;
                    if result_update{
                        return Some(LoginInfoDTO {
                            username: user.user_name,
                            login_session: login_session_str,
                        });
                    }
                    else {
                        return  None;
                    }
                 }else {
                     return None;
                 }

             }
             else {
                return Some(LoginInfoDTO {
                    username: user.user_name,
                    login_session: String::new(),
                });
             }
            },
            None=> return None,
        }
    }

    //logout
    pub async fn logout(user_id:i32,pool:MySqlPool){
          let query_str=format!("select * from users where id={}",user_id);
          let user=sqlx::query(&query_str)
                        .map(|row:MySqlRow|User{
                            id:row.get("id"),
                            email:row.get("email"),
                            user_name:row.get("username"),
                            password:row.get("password"),
                            login_session:row.get("login_session")
                        })
                        .fetch_one(&pool.clone())
                        .await
                        .ok();
         match user {
            Some(user) => {
                Self::update_login_session(user.user_name,"".into(),pool);
            },
            None => todo!(),
        }
    }

    //is valid login session
    pub async fn is_valid_login_session(user_token:UserToken,pool:&MySqlPool)->bool{
         let query_str=format!("select * from users where username='{}' and login_session ='{}'",user_token.user,user_token.login_session);

         let user=sqlx::query(&query_str)
                                                        .execute(&pool.clone())
                                                        .await
                                                        .is_ok();
           if user {
            true
           }
           else {
               false
           }
    }
    //update login session
    pub async fn update_login_session(user_name:String,login_sesion:String,pool:MySqlPool)->bool{
        let user=Self::find_user_by_username(user_name.clone(), pool.clone());
        if let Ok(user)=Self::find_user_by_username(user_name.clone(), pool.clone()).await{
            let query_str=format!("update users set login_session='{}' where id={}",login_sesion,user.id);
                sqlx::query(&query_str)
                     .execute(&pool.clone())
                     .await
                     .is_ok()
        }
        else {
            false
        }
    }
    pub fn generate_login_session()->String{
        Uuid::new_v4().to_string()
    }
    //get all user
    pub async fn get_all_users(pool:MySqlPool)->Result<Vec<User>,Error>{
       let lst_user=sqlx::query("SELECT * FROM Users")
                            .map(|row:MySqlRow|User{
                                id:row.get("id"),
                                email:row.get("email"),
                                user_name:row.get("username"),
                                password:row.get("password"),
                                login_session:row.get("login_session")
                            })
                            .fetch_all(&pool.clone())
                            .await;
            lst_user
    } 

    //find user by username
    pub async fn find_user_by_username(user_name:String,pool:MySqlPool)->Result<User,Error>{
      let query_str = format!("SELECT * FROM Users WHERE username='{}'",&user_name);
      let user=sqlx::query(&query_str)
                                            .map(|row:MySqlRow|User{
                                                id:row.get("id"),
                                                email:row.get("email"),
                                                user_name:row.get("username"),
                                                password:row.get("password"),
                                                login_session:row.get("login_session")
                                            })
                                            .fetch_one(&pool.clone())
                                            .await;                                                                
       user                                        
    }

}