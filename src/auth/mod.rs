use axum::{
    headers::Cookie as ACookie,
    response::{Html, IntoResponse, Json, Redirect, Response, Result},
    routing::{delete, get, on, patch, post, put},
    BoxError, Error, Router,
};
use tower_cookies::{
    Key, SignedCookies, PrivateCookies, 
    Cookie, Cookies, CookieManagerLayer, CookieManager,
};

pub fn routes() -> axum::Router {
    Router::default()
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/logout", post(logout))
        .layer(CookieManagerLayer::new())
}

pub async fn login(cookies: Cookies) -> Response {
    let key = Key::generate();
    let success = Cookie::new("9753PERLI83", "logged-in");
    cookies.private(&key).add(success);
    
    Html("<h1>Login<h1>").into_response()
}

pub async fn signup() -> Response {
    let key = Key::generate();
    Html("<h1>Signup<h1>").into_response()
}

pub async fn logout(cookies: Cookies) -> Json<bool> {
    if let Some(authc) = cookies.private(key: &Key.get("9753PERLI83") {
        match authc.value() {
            "logged-in" => {
                cookies.remove(auth_cookie);
                return Json(true);
            },
            _ => return Json(false),
        }
    }
    return Json(false);
}
