use crate::cmd_likes::handle_likes;
use crate::cmd_posts::handle_posts;
use crate::commands::{Cli, Commands};
use clap::Parser;
use std::process::ExitCode;

mod chost_json;
mod commands;
mod cmd_likes;
mod cmd_posts;
mod chost_formatter;

fn main() -> ExitCode {
	let cli = Cli::parse();

	let result = match &cli.command {
		Commands::Likes(args) => {
			handle_likes(&args.input, &args.output)
		}
		Commands::Posts(args) => {
			handle_posts(&args.input, &args.output)
		}
	};

	match result {
		Ok(_) => {
			println!("done!");
			ExitCode::SUCCESS
		}
		Err(e) => {
			eprintln!("execution failed: {e:#?}");
			ExitCode::FAILURE
		}
	}
}

