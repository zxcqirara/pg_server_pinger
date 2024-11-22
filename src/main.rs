use colored::Colorize;
use dotenv::dotenv;
use std::env;
use std::net::{TcpStream};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{error, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	dotenv().ok();                 // Init .env file
	tracing_subscriber::registry() // Init logger
		.with(
			tracing_subscriber::EnvFilter::try_from_default_env()
				.unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into())
		)
		.with(tracing_subscriber::fmt::layer())
		.init();

	// Get some vars from env
	let target_host = env::var("TARGET_HOST")
		.expect("TARGET_HOST is not set");

	let target_port: u16 = env::var("TARGET_PORT")
		.expect("TARGET_PORT is not set")
		.parse()
		.expect("TARGET_PORT is not a number");

	let delay_secs: u64 = env::var("DELAY_SECS")
		.expect("DELAY_SECS is not set")
		.parse()
		.expect("DELAY_SECS is not a u64 number");

	info!("{}", format!("Checking {target_host} with TPC pings...").blue());

	// Init timeouts and delays
	let timeout = Duration::from_secs(10);
	let delay_secs = Duration::from_secs(delay_secs);
	let mut is_last_ping_success = false;

	// Main check loop
	loop {
		match ping_server(&target_host, target_port, timeout) {
			Ok(delay) if !is_last_ping_success => {
				info!("{}", format!("Success: {:0.2?}", delay).green());
				is_last_ping_success = true;
			},
			Err(e) if is_last_ping_success => {
				error!("{}", format!("Failed to ping the server: {}", e).red());
				is_last_ping_success = false;
			},
			_ => {}
		}

		sleep(delay_secs).await;
	}
}

// This function is written by ChatGPT lol
fn ping_server(host: &str, port: u16, timeout: Duration) -> anyhow::Result<Duration> {
	let address = format!("{}:{}", host, port);
	let start = Instant::now();

	// Attempt to connect to the server
	let connection_result = TcpStream::connect_timeout(&address.parse()?, timeout);

	match connection_result {
		Ok(_) => {
			// If connection succeeds, calculate the elapsed time
			Ok(start.elapsed())
		}
		Err(e) => {
			// If connection fails, return error
			Err(e.into())
		}
	}
}