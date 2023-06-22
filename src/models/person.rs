use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    mysql::{MySqlQueryResult, MySqlRow},
    Error, FromRow, MySqlPool, Row,
};
use std::result;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub gender: i8,
    pub age: i32,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub user_id: i32,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PersonDTO {
    pub name: String,
    pub gender: i8,
    pub age: i32,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub user_id: i32,
}

impl Person {
    pub async fn find_all(pool: MySqlPool) -> Option<Vec<Person>> {
        let query_str = format!("Select * from person");
        let result = sqlx::query(&query_str)
            .map(|row: MySqlRow| Person {
                id: row.get("id"),
                email: row.get("email"),
                name: row.get("name"),
                gender: row.get("gender"),
                address: row.get("address"),
                age: row.get("age"),
                phone: row.get("phone"),
                user_id: row.get("user_id"),
            })
            .fetch_all(&pool)
            .await
            .ok();
        result
    }
    pub async fn find_person_by_id(personid: i32, pool: MySqlPool) -> Option<Person> {
        let query_str = format!("Select * from person Where id={}", personid);
        let result = sqlx::query(&query_str)
            .map(|row: MySqlRow| Person {
                id: row.get("id"),
                email: row.get("email"),
                name: row.get("name"),
                gender: row.get("gender"),
                address: row.get("address"),
                age: row.get("age"),
                phone: row.get("phone"),
                user_id: row.get("user_id"),
            })
            .fetch_one(&pool)
            .await
            .ok();
        result
    }
    //create user after update info person
    pub async fn create(user_id: i32, person: PersonDTO, pool: MySqlPool) -> bool {
        let query_str1=format!("Insert into person (name,gender,age,address,phone,email,user_id) values ('{}','{}',{},'{}','{}','{}',{})",person.name,person.gender,person.age,person.address,person.phone,person.email,person.user_id);
        sqlx::query(&query_str1).execute(&pool).await.is_ok()
    }
    pub async fn delete(personid: i32, pool: MySqlPool) -> bool {
        let query_str1 = format!("Delete person where id={}", personid);
        sqlx::query(&query_str1).execute(&pool).await.is_ok()
    }
    pub fn update(personid: i32, person: PersonDTO, pool: MySqlPool) -> bool {
        todo!();
    }
}
