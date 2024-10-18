use crate::cohost_json::Chost;
use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[allow(unused_variables)]
pub(crate) fn handle_likes<P: AsRef<Path>>(in_file: P, out_dir: P) -> Result<()> {
	let file = File::open(in_file)?;
	let reader = BufReader::new(file);

	for (i, line) in reader.lines().enumerate() {
		let line = line?;
		let chost: Chost = serde_json::from_str(&line)
			.inspect_err(|e| eprintln!("Deserialization failed: {e:?}"))?;
		println!("[{:>03}] {} (@{})", i + 1, chost.filename, chost.posting_project.handle);
	}

	Ok(())
}