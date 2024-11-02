use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
pub(crate) struct MoreArgs {
	#[arg(short, help = "The input JSON file")]
	pub input: PathBuf,
	#[arg(help = "The output folder to put .md files into")]
	pub output: PathBuf,
}

#[derive(Parser, Debug)]
pub(crate) struct TestArgs {
	#[arg(short, help = "The input JSON file")]
	pub input: PathBuf,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
	#[command(about = "Uses liked.json as a base")]
	Likes(MoreArgs),

	#[command(about = "Uses posts.json as a base")]
	Posts(MoreArgs),

	#[command(about = "Loads a single post from a JSON file to test the parser")]
	Test(TestArgs),
}

#[derive(Parser, Debug)]
#[command(about = "Converts cohost-dl output to Markdown files for Obsidian")]
pub(crate) struct Cli {
	#[command(subcommand)]
	pub command: Commands,

	#[arg(long, global = true)]
	pub debug: bool,
}