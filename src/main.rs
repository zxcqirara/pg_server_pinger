use colored::Colorize;
use dotenv::dotenv;
use std::env;
use std::net::{TcpStream};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	dotenv().ok();      // Init .env file
	env_logger::init(); // Init logger

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

	info!("{}", format!("Checking {target_host}...").blue());

	// Init timeouts and delays
	let timeout = Duration::from_secs(10);
	let delay_secs = Duration::from_secs(delay_secs);
	let mut is_last_ping_success = false;

	// Main check loop
	loop {
		match ping_server(&target_host, target_port, timeout) {
			Some(delay) if !is_last_ping_success => {
				info!("{}", format!("Success: {:0.2?}", delay).green());
				is_last_ping_success = true;
			},
			None if is_last_ping_success => {
				error!("{}", "Failed to ping the server".red());
				is_last_ping_success = false;
			},
			_ => {}
		}

		sleep(delay_secs).await;
	}
}

// This function is written by ChatGPT lol
fn ping_server(host: &str, port: u16, timeout: Duration) -> Option<Duration> {
	let address = format!("{}:{}", host, port);
	let start = Instant::now();

	// Attempt to connect to the server
	let connection_result = TcpStream::connect_timeout(&address.parse().unwrap(), timeout);

	match connection_result {
		Ok(_) => {
			// If connection succeeds, calculate the elapsed time
			Some(start.elapsed())
		}
		Err(_) => {
			// If connection fails, return None
			None
		}
	}
}