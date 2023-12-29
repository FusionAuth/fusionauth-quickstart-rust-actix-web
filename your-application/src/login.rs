use actix_web::{get, HttpResponse, Responder};
use dotenv::dotenv;
use std::env;

#[get("/login")]
async fn login() -> impl Responder {
    dotenv().ok();
    let c = env::var("FUSIONAUTH_CLIENT_ID").expect("FUSIONAUTH_CLIENT_ID not found");
    println!("{}", c);
    HttpResponse::Ok().body("login")
}