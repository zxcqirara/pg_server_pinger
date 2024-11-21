use axum::{
	routing::get,
	Router,
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
	dotenv().ok();

	// build our application with a single route
	let app = Router::new()
		.route("/", get(|| async { "Hello, World!" }));

	// run our app with hyper, listening globally on port 3000
	let port = std::env::var("PORT")
		.expect("PORT environment variable not set");

	let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
	println!("Listening on {}", listener.local_addr().unwrap());
	
	axum::serve(listener, app).await.unwrap();
}