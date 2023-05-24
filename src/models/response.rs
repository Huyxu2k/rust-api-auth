use serde::{Serialize, Deserialize};


#[derive(Deserialize,Serialize,Debug)]
pub struct ResponseBody<T>{
  pub message:String,
  pub data:T,
}
impl <T> ResponseBody<T> {
    pub fn new(message:String,data:T)->ResponseBody<T>{
        ResponseBody {
             message:message,
             data,
            }
    }
}
