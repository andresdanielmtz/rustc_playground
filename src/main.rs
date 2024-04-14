use axum::{
    Router,
    routing::get,
};
mod func;

#[tokio::main]
async fn main() {
    const PORT: i16 = 3005; // 3000 is busy.
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!\n"}))
        .route("/random", get(|| async { func::get_random_number().to_string() }));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT)).await.unwrap();

    println!("Listening on port {}!", PORT);
    axum::serve(listener, app).await.unwrap();
}

