
use axum::Router;

use webapp::{establish_diesel_pool, establish_sqlx_conn};

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use webapp::{comm,dbsqlx,org,dbdiesel,user};





#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // set up connection pool
    let sqlx_pool: sqlx::Pool<sqlx::Postgres> = establish_sqlx_conn().await;

    let diesel_pool = establish_diesel_pool();

    // build our application with a route
    let app = Router::new()
        .merge(user::app())
        .merge(org::app())
        .merge(comm::app())
        .merge(dbsqlx::app(axum::extract::State(sqlx_pool)))
        .merge(dbdiesel::app(diesel_pool))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
