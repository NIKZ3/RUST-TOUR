/*
Actix_web global object shared among coroutines.
 */

 use actix_web::{get, web::Json, web::Data, App, HttpResponse, HttpServer, Responder};
 use tokio::time::{self, Duration};
 use actix_rt;
 use std::sync::{Arc, Mutex};
 use lazy_static::lazy_static;
 
 
 lazy_static! {
     static ref MY_DATA: Arc<Mutex<i32>> = Arc::new(Mutex::new(5));
 }
 
 
 async fn my_coroutine(value: Arc<Mutex<i32>>) {
     println!("Running my coroutine");
     let mut interval = actix_rt::time::interval(Duration::from_secs(10));
     loop {
         println!("strong, {:#?}",  Arc::strong_count(&value));
         interval.tick().await; 
     }
 }
 
 #[get("/")]
 async fn hello(value: Data<Arc<Mutex<i32>>>) -> Json<String> {
     println!("strong2, {:#?}",  &value);
     return Json("HELLO".to_string())
 }
 
 
 
 #[actix_web::main]
 async fn main() -> std::io::Result<()> {
     //let x = 20;
     println!("Start thread, {:#?}",  Arc::strong_count(&MY_DATA));
     //let MY_DATA =  Arc::new(Mutex::new(x));
     actix_rt::spawn(my_coroutine(Arc::clone(&MY_DATA)));
     println!("Start thread, {:#?}",  Arc::strong_count(&MY_DATA));
     HttpServer::new(|| {
         println!("Starting thread");
         println!("Start thread, {:#?}",  Arc::strong_count(&MY_DATA));
         App::new()
             .app_data(Data::new(Arc::clone(&MY_DATA)))
             .service(hello) 
     })
     .bind("127.0.0.1:8080")?
     .run()
     .await
 }