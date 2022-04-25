pub use mongodb::bson::{doc, Document};
pub use mongodb::{options::ClientOptions, Client};


pub async fn post_thread_logic(user_ip:String,tag1:String,tag2:String,tag3:String,post:String,tital:String,client:&Client)->Result<Ok(),Error()>
{
   let db = Client.database("thread");
   let collection = db.Collection::<Document>("thread");
   let data = doc!{
       "Thread_name":tital,
       "by":user_ip,
       "thread_content":post,
       "likes":0
       "dislikes":0,
       "comment_numbers":0,
       "comments":[{}]
       };
  collection.insert_one(data,None);

}
