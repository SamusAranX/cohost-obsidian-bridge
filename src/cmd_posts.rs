#![allow(unused_variables)]

use crate::chost_json::Chost;
use anyhow::Result;
use std::fs;
use std::path::Path;

#[allow(dead_code)]
fn load_posts_json<P: AsRef<Path>>(in_file: P) -> Result<Vec<Chost>> {
	let chosts_json = fs::read_to_string(in_file)?;
	let chosts: Vec<Chost> = serde_json::from_str(&chosts_json)
		.inspect_err(|e| eprintln!("Deserialization failed: {e:?}"))?;

	Ok(chosts)
}

#[allow(dead_code)]
fn load_posts_js<P: AsRef<Path>>(in_file: P) -> Result<Vec<Chost>> {
	unimplemented!()
}

pub(crate) fn handle_posts<P: AsRef<Path>>(in_file: P, out_dir: P) -> Result<()> {
	let chosts_json = fs::read_to_string(in_file)?;

	let chosts: Vec<Chost> = serde_json::from_str(&chosts_json)
		.inspect_err(|e| eprintln!("Deserialization failed: {e:?}"))?;

	for (i, chost) in chosts.iter().enumerate() {
		println!("[{:>03}] {} (@{})", i + 1, chost.filename, chost.posting_project.handle);
	}

	Ok(())
}