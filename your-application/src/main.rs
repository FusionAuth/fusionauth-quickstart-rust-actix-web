use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;

#[get("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
    HttpServer::new(|| {
        App::new()
            .service(login)
            .service(fs::Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(|| async { fs::NamedFile::open("static/index.html") }))
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