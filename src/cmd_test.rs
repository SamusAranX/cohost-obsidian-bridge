use crate::chost_json::ChostContainerTest;
use anyhow::Result;
use std::fs;
use std::path::Path;

#[allow(unused_variables)]
pub(crate) fn test_post<P: AsRef<Path>>(in_file: P) -> Result<()> {
	let chosts_json = fs::read_to_string(in_file)?;
	let chost_test: ChostContainerTest = serde_json::from_str(&chosts_json)
		.inspect_err(|e| eprintln!("Deserialization failed: {e:?}"))?;

	Ok(())
}