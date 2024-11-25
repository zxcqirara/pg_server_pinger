use colored::Colorize;
use dotenv::dotenv;
use std::env;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{ChatId, ParseMode};
use tokio::time::sleep;
use tracing::{error, info, warn};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	dotenv().ok();                 // Init .env file

	// Init logger
	let logger_timer = ChronoLocal::new("%Y/%m/%d %H:%M:%S".to_owned());
	tracing_subscriber::registry()
		.with(
			tracing_subscriber::EnvFilter::try_from_default_env()
				.unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into())
		)
		.with(
			tracing_subscriber::fmt::layer()
				.with_timer(logger_timer)
		)
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

	let high_delay_threshold_millis: u64 = env::var("HIGH_DELAY_THRESHOLD_MILLIS")
		.expect("HIGH_DELAY_THRESHOLD_MILLIS is not set")
		.parse()
		.expect("HIGH_DELAY_THRESHOLD_MILLIS is not a u64 number");

	let telegram_token = env::var("TELEGRAM_TOKEN")
		.expect("TELEGRAM_TOKEN is not set");

	let telegram_group_id: i64 = env::var("TELEGRAM_GROUP_ID")
		.expect("TELEGRAM_GROUP_ID is not set")
		.parse()
		.expect("TELEGRAM_GROUP_ID is not a number");
	let telegram_group_id = ChatId(telegram_group_id);

	info!("Initializing bot...");

	let bot = teloxide::Bot::new(telegram_token);

	{ // getme check
		let me = bot.get_me().await.expect("Failed to initialize bot");
		let username = me.username.clone().expect("Failed to fetch bot username");

		info!("Bot initialized: {}", format!("@{username}").cyan());
	}

	info!("Checking {target_host} with TPC pings...");

	// Init timeouts and delays
	let timeout = Duration::from_secs(10);
	let delay_secs = Duration::from_secs(delay_secs);
	let mut is_last_ping_success = None;

	// Main check loop
	loop {
		match ping_server(&target_host, target_port, timeout) {
			Ok(delay) if is_last_ping_success.is_none_or(|b: bool| !b) => {
				let formatted_delay = format!("{:0.2?}", delay);

				if delay > Duration::from_millis(high_delay_threshold_millis) {
					warn!("{}", format!("High ping: {formatted_delay}").yellow());

					bot.send_message(telegram_group_id, format!("Повышенный пинг на сервере: *{formatted_delay}*").replace(".", "\\."))
						.parse_mode(ParseMode::MarkdownV2)
						.await?;
				} else {
					info!("{}", format!("Success: {formatted_delay}").green());
				}

				is_last_ping_success = Some(true);
			},
			Err(e) if is_last_ping_success.is_none_or(|b| b) => {
				error!("{}", format!("Failed to ping the server: {}", e).red());

				bot.send_message(telegram_group_id, format!("Произошла критическая апшыпка:\n```\n{e:?}\n```"))
					.parse_mode(ParseMode::MarkdownV2)
					.await?;

				is_last_ping_success = Some(false);
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