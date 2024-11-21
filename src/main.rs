use axum::{
	routing::get,
	Router,
};

#[tokio::main]
async fn main() {
	// build our application with a single route
	let app = Router::new()
		.route("/", get(|| async { "Hello, World!" }));

	// run our app with hyper, listening globally on port 3000
	let listener = tokio::net::TcpListener::bind("0.0.0.0:20042").await.unwrap();
	println!("Listening on {}", listener.local_addr().unwrap());
	
	axum::serve(listener, app).await.unwrap();
}