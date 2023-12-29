use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use handlebars::Handlebars;
use std::collections::HashMap;
// use std::env; //todo

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
    // let data = json!({
    //     "name": "Handlebars"
    // });
    let mut data = HashMap::new();
    data.insert("name", "Handlebars");
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let handlebars_ref = setup_handlebars().await;
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(login)
            .service(logout)
            .service(fs::Files::new("/static", "static").show_files_listing())
            // .route("/", web::get().to(|| async { fs::NamedFile::open("static/index.html") }))
            .app_data(handlebars_ref.clone())
            // .handler(
            //     "/",
            //     fs::StaticFiles::new("./static/").unwrap().index_file("index.html")
            // )
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 9012))?
    .run()
    .await
}


async fn setup_handlebars() -> web::Data<Handlebars<'static>> {

    // let current_dir = env::current_dir().unwrap();
    // println!("Current directory: {:?}", current_dir);
    // let templates_path = "./templates";
    // println!("Templates path: {:?}", templates_path);

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();
    web::Data::new(handlebars)
}

// println!("Request received on the index route");