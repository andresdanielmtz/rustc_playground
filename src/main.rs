use axum::{
    routing::get,
    Router,
};
mod func;

#[tokio::main]
async fn main() {
    const PORT: i32 = 3005;
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!\n"}))
        .route("/random", get(|| async { func::get_random_number().to_string() }));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT)).await.unwrap();

    println!("{}", format!("It is currently working on the port {}!", PORT));
    axum::serve(listener, app).await.unwrap();
}

