use actix_web::{get, route, web, http, App, HttpRequest, HttpResponse, HttpServer}; // web server
use actix_files as fs; // static image files
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore}; // store auth info in browser cookies
use actix_web::cookie::Key;
use handlebars::Handlebars; // html templates
use std::collections::HashMap; // pass data to templates
use dotenv::dotenv; // load .env file
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let handlebars_ref = setup_handlebars().await;
    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), Key::generate().clone()))
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

async fn setup_handlebars() -> web::Data<Handlebars<'static>> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();
    web::Data::new(handlebars)
}

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>, session: Session) -> HttpResponse {
    if let Ok(Some(_)) = session.get::<String>("email") {
        return HttpResponse::Found().append_header(("Location", "/account")).finish();
    }
    let body = hb.render("index", &{}).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/account")]
async fn account(hb: web::Data<Handlebars<'_>>, session: Session) -> HttpResponse {
    if let Ok(None) | Err(_) = session.get::<String>("email") {
        return HttpResponse::Found().append_header(("Location", "/")).finish();
    }
    let mut data = HashMap::new();
    data.insert("email", session.get::<String>("email").unwrap());
    let body = hb.render("account", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[route("/change", method="GET", method="POST")]
async fn change(req: HttpRequest, hb: web::Data<Handlebars<'_>>, session: Session) -> HttpResponse {
    if let Ok(None) | Err(_) = session.get::<String>("email") {
        return HttpResponse::Found().append_header(("Location", "/")).finish();
    }
    let mut data = HashMap::<&str, String>::new();
    data.insert("email", session.get::<String>("email").unwrap().unwrap());
    if req.method() == http::Method::GET {
        data.insert("is_get_request", "true".to_string());
    }
    else if req.method() == http::Method::POST {
        data.insert("is_get_request", "false".to_string());
    }
    else {
        return HttpResponse::BadRequest().finish();
    }
    let body = hb.render("change", &data).unwrap();
    HttpResponse::Ok().body(body)
}

// todo crsf

fn calculate_change(amount: &str) -> Result<HashMap<&'static str, String>, &'static str> {
    let total = amount.parse::<f64>().map_err(|_| "Invalid input")?;
    let rounded_total = (total * 100.0).floor() / 100.0;

    let mut state = HashMap::new();
    state.insert("iserror", (!amount.chars().all(char::is_numeric)).to_string());
    state.insert("hasChange", "true".to_string());
    state.insert("total", format!("{:.2}", rounded_total));

    let nickels = (rounded_total / 0.05).floor();
    state.insert("nickels", format!("{}", nickels));

    let pennies = ((rounded_total - (0.05 * nickels)) / 0.01).ceil();
    state.insert("pennies", format!("{}", pennies));

    Ok(state)
}

