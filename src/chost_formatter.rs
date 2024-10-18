use crate::chost_json::Chost;
use anyhow::Result;
use chrono::Local;
use std::fmt::Write;

// ---
// date: 2024-09-23T04:25:00
// cohost/users:
//   - nex3
//   - eramdam
//   - jkap
//   - TarotCard2
// cohost/tags:
//   - ask jae anything
// cohost/original-post: https://cohost.org/nex3/post/7807131-div-style-display
// ---

impl Chost {
	#[allow(dead_code)]
	pub(crate) fn generate_markdown(&self) -> Result<String> {
		let mut markdown = String::new();

		// write obsidian front matter
		write!(&mut markdown, "---")?;
		write!(&mut markdown, "date: {}", self.published_at.with_timezone(&Local).to_rfc3339())?;
		write!(&mut markdown, "cohost/users:")?;
		write!(&mut markdown, "---")?;

		Ok("".parse().unwrap())
	}
}