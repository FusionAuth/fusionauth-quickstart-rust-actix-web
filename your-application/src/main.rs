#![allow(unused_imports)] // todo remove
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder}; // web server
use actix_files as fs; // static image files
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore}; // store auth info in browser cookies
use actix_web::cookie::Key;
use handlebars::Handlebars; // html templates
use std::collections::HashMap; // pass data to templates
use dotenv::dotenv; // load .env file
mod auth;

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let body = hb.render("index", &{}).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/account")]
async fn account(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let mut data = HashMap::new();
    data.insert("email", "todo@example.com");
    let body = hb.render("account", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/change")]
async fn change(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let mut data = HashMap::new();
    data.insert("email", "todo@example.com");
    let body = hb.render("change", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let secret_key = Key::generate();
    let handlebars_ref = setup_handlebars().await;
    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), secret_key.clone()))
            // .wrap(CookieSession::signed(get_session_key()).secure(false)) // change secure to true (https) in production
            .service(account)
            .service(change)
            .service(index)
            .service(auth::login)
            .service(auth::logout)
            .service(auth::callback)
            .service(fs::Files::new("/static", "static").show_files_listing())
            .app_data(handlebars_ref.clone())
    })
    .bind(("127.0.0.1", 9012))?
    .run()
    .await
}

// fn get_session_key() -> [u8; 32] {
//     let bytes = "a-secret-session-key-no-one-can-guess".as_bytes(); // change this in production
//     let mut key_bytes = [0u8; 32];
//     key_bytes[..bytes.len()].copy_from_slice(&bytes[..bytes.len().min(32)]);
//     key_bytes
// }

async fn setup_handlebars() -> web::Data<Handlebars<'static>> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();
    web::Data::new(handlebars)
}
