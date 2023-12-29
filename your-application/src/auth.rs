use actix_web::{get, HttpResponse, Responder};
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
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;

#[get("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("logout")
}

#[get("/login")]
async fn login() -> impl Responder {
    let client = get_oauth_client();
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // .add_scope(Scope::new("read".to_string()))
        // .add_scope(Scope::new("write".to_string()))
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    // HttpResponse::Ok().body("login")
    HttpResponse::Found().header("Location", auth_url.to_string()).finish()
}

#[get("/callback")]
async fn callback() -> impl Responder {
    let c = env::var("FUSIONAUTH_CLIENT_ID").expect("FUSIONAUTH_CLIENT_ID not found");
    println!("{}", c);
    HttpResponse::Ok().body("login")
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



// // Once the user has been redirected to the redirect URL, you'll have access to the
// // authorization code. For security reasons, your code should verify that the `state`
// // parameter returned by the server matches `csrf_state`.

// // Now you can trade it for an access token.
// let token_result = client
//     .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
//     // Set the PKCE code verifier.
//     .set_pkce_verifier(pkce_verifier)
//     .request_async(async_http_client)
//     .await?;
