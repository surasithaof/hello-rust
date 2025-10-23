use axum::Router;

mod todo;

#[tokio::main]
async fn main() {
    let router = Router::new();
    let router = router.merge(todo::init_router());

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);
    axum::serve(listener, router).await.unwrap();
}
