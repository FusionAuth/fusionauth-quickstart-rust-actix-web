use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use handlebars::Handlebars;
use std::collections::HashMap;

#[get("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("login")
}

#[get("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("logout")
}

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let handlebars_ref = setup_handlebars().await;
    HttpServer::new(move || {
        App::new()
            .service(login)
            .service(logout)
            .service(index)
            .service(account)
            .service(fs::Files::new("/static", "static").show_files_listing())
            // .route("/", web::get().to(|| async { fs::NamedFile::open("static/index.html") }))
            .app_data(handlebars_ref.clone())
    })
    .bind(("127.0.0.1", 9012))?
    .run()
    .await
}

async fn setup_handlebars() -> web::Data<Handlebars<'static>> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();
    web::Data::new(handlebars)
}