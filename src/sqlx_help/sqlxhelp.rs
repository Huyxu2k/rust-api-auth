use std::result;
use chrono::{Utc,DateTime, NaiveDate };
use sqlx::{MySqlPool, Error,FromRow,Row, mysql::{MySqlRow}};
use serde::{Deserialize,Serialize};



#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct QueryInit <T>{
    data:T,
    table:String
}
impl <T> QueryInit <T> where

{
    pub fn insert(data:T,table:String)->String{
        //let query_str=format!("INSERT INTO {table} ({data}) values ({}) ");
        todo!();
    }
    pub fn update(data:T)->String{
        todo!();
    }
    pub fn delete(data:T)->String{
       todo!();
    }
}