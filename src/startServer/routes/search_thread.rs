use serde::{Deserialize};
#[derive(Deserialize)]
pub struct query
{
    by:String,
    tag1:String,
    tag2:String,
    tag3:String,
    thread_name:String
}



pub async fn search(userData: web::Query<query>)
{
   
}