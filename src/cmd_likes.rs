use crate::chost_json::Chost;
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
		let chost: Chost = serde_json::from_str(&line)?;
		println!("[{:>03}] {} (@{})", i + 1, chost.filename, chost.posting_project.handle);

		let markdown = chost.generate_markdown()?;
		println!("{markdown}");

		// debug: abort after the 7th chost
		if i + 1 == 7 {
			break;
		}
	}

	Ok(())
}