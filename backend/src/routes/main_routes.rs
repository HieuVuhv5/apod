use axum::response::Response;
use axum::routing::*;
use axum::Router;
use http::StatusCode;
use hyper::Body;
use sqlx::PgPool;

use crate::db::Store;
use crate::handlers::root;
//use crate::routes::comment_routes::comment_routes;
use crate::{handlers, layers};

pub async fn app(pool: PgPool) -> Router {
    let db = Store::with_pool(pool);

    let (cors_layer, trace_layer) = layers::get_layers();

    Router::new()
        // The router matches these FROM TOP TO BOTTOM explicitly!
        .route("/", get(root))
        .route("/apod/:apod_date", get(handlers::get_apod_by_date))
        .route("/apod", get(handlers::apod_pages))
        .route("/gallery",get(handlers::gallery_page))
        .route("/slideshow", get(handlers::slideshow_page))
        .route("/apods", get(handlers::get_all_apod))
        .route("/add_gallery", post(handlers::add_gallery))
        .route("/add_gallery_apod", post(handlers::add_gallery_from_apod))
        .route("/delete_gallery", post(handlers::delete_gallery).get(handlers::gallery_page))
        .route("/apod", post(handlers::create_apod))
       // .route("/users", post(handlers::register))
        .route("/login", post(handlers::login).get(root))
        .route("/protected", get(handlers::protected))
        .route("/search", post(handlers::search_page))
        .route("/get_register",get(handlers::get_register))
        .route("/register",post(handlers::register))
        .route("/logout", get(handlers::logout).post(root))

        .route("/*_", get(handle_404))
        //.merge(comment_routes())
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(db)
}

async fn handle_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The requested page could not be found"))
        .unwrap()
}
