use actix_web::{get, web, Error, HttpResponse, Responder};
use actix_session::{Session};
use std::env;
use anyhow;
use url::Url;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    PkceCodeVerifier,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
// use oauth2::reqwest::http_client;
use serde::Deserialize;

#[get("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("logout")
}

#[get("/login")]
async fn login(session: Session) -> impl Responder {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = get_oauth_client()
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    session.insert("csrf_token",    csrf_token);
    session.insert("pkce_verifier", pkce_verifier);
    // HttpResponse::Found().header("Location", auth_url.to_string()).finish()
    HttpResponse::Found().append_header(("Location", auth_url.to_string())).finish()
}

#[derive(Deserialize)]
struct AuthCallbackParams {
    state: String,
    code: String,
}

//Result<HttpResponse, Error>
#[get("/callback")]
async fn callback(params: web::Query<AuthCallbackParams>, session: Session) ->  impl Responder {
    //callback url from fusionauth: http://localhost:9012/callback?code=4u73uE6d-xbJ5lTBeY2z7cCXMWMEYP_TSB7v_vOdJp4&locale=en&state=XPNMGQfsHPKNP7PK7qyJyg&userState=Authenticated
    let received_state = &params.state;
    if let Ok(saved_state) = session.get::<String>("csrf_token") {
        if saved_state != Some(received_state.clone()) {
            return HttpResponse::BadRequest().body("PKCE state mismatch");
        }
    }
    else {
        return HttpResponse::InternalServerError().body("Session error")
    }
    let pkce_verifier = session.get::<String>("pkce_verifier").unwrap().unwrap();
    let token_result = get_oauth_client()
        .exchange_code(AuthorizationCode::new(params.code.clone()))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
        // .request(http_client);
        .request_async(async_http_client).await?;
    // {
    //     Ok(result) => result,
    //     Err(e) => {
    //         return Ok(HttpResponse::InternalServerError().body("Internal server error"));
    //     }
    // };
    // session.insert("email", token_result.email.to_string().unwrap());
    HttpResponse::Found().append_header(("Location", "/account")).finish()
}

fn get_oauth_client() -> BasicClient {
    BasicClient::new(
        ClientId::new(env::var("FUSIONAUTH_CLIENT_ID").expect("Missing FUSIONAUTH_CLIENT_ID")),
        Some(ClientSecret::new(env::var("FUSIONAUTH_CLIENT_SECRET").expect("Missing FUSIONAUTH_CLIENT_SECRET"))),
        AuthUrl::new(env::var("FUSIONAUTH_BROWSER_URL").expect("Missing FUSIONAUTH_BROWSER_URL") + "/oauth2/authorize").expect("Invalid AuthUrl"),
        Some(TokenUrl::new(env::var("FUSIONAUTH_SERVER_URL").expect("Missing FUSIONAUTH_SERVER_URL") + "/oauth2/token").expect("Invalid TokenUrl"))
    )
    .set_redirect_uri(RedirectUrl::new(env::var("FUSIONAUTH_REDIRECT_URL").expect("Missing FUSIONAUTH_REDIRECT_URL")).expect("Invalid RedirectUrl"))
}