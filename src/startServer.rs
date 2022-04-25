use actix_web:: {web, App, HttpServer};
use mongodb::Client;
use actix_web::dev:: Server;
use std::net::TcpListener;
mod routes;
use routes::*;


//we send connection to databse from main function to the run function 
pub fn run(listener:TcpListener,client:Client) -> Result<Server, std::io:: Error> 
     {
         let client = web::Data:: new(client);
         let server = HttpServer:: new(move|| {
         App:: new()
         .route("/health_check", web:: post().to(post_thread))
         .app_data(client.clone())
       }).listen(listener)?
       .run();

Ok(server)
}