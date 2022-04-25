use actix_web::*;
use mongodb::Client;

use crate::collection::Post_thread_logic::*;


use serde::{Deserialize};
#[derive(Deserialize)]
pub struct thread{
    post:String,
    tag1:String,
    tag2:String,
    tag3:String
    
}

pub async fn post_thread(req: HttpRequest,form: web::Form<thread>,client : web::Data<Client>)->impl Responder
{
   //capture if user's Ip adderess
   let user_ip = format!("{:?}",req.peer_addr());
   //check if user is blocked or not
   let check:bool = is_blocked(user_ip);
   if check == true
   {
       return HttpRequest.Error().finish();
   }
   else
   {
       match post_thread_logic(user_ip,tag1,tag2,tag3,post, client)
       {
           Ok(_) =>HttpResponse.Ok().finish(),
           Error(e) => HttpRequest.Error().finish()
       }
   }
}

   

