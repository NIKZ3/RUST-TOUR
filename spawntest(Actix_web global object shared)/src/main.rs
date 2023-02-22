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
        interval.tick().await;
        // do something
        let mut data = value.lock().unwrap();

        // Modify the shared data
        *data += 1;
        println!("Running my coroutine Alright, {:#?}", data);
    }
}

#[get("/")]
async fn hello(value: Data<Arc<Mutex<i32>>>) -> Json<String> {
    let mut data = value.lock().unwrap(); 
    *data += 1;
    println!("OK1 {:#?}", data);
    return Json("HELLO".to_string())
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_rt::spawn(my_coroutine(MY_DATA.clone()));
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(MY_DATA.clone()))
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}