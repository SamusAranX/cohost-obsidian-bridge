use crate::chost_json::{Ask, Attachment, Block, Chost, Markdown};
use anyhow::Result;
use chrono::{Local, Locale};
use indexmap::{indexset, IndexSet};
use std::fmt::Write;

// >[!cohost-share] Natalie
// >@nex3 <time datetime="2024-09-23T04:25:43.380+02:00">Mon, Sep 23, 2024 at 4:25 AM</time> :LiRefreshCw: **Damien** @eramdam
//
// >[!cohost-post] jae
// >@jkap <time datetime="2024-09-23T03:23:34.628+02:00">Mon, Sep 23, 2024 at 3:23 AM</time>
//
// >[!cohost-ask] [@TarotCard2](https://cohost.org/TarotCard2) asked:
// >Are we allowed to use the colorscheme from here on cohost elsewhere? If so, what are the official hex codes?

pub(crate) trait Renderable {
	fn render<W: Write>(&self, receiver: &mut W) -> Result<()>;
}

impl Renderable for Markdown {
	fn render<W: Write>(&self, receiver: &mut W) -> Result<()> {
		writeln!(receiver, "{}", self.content.trim())?;
		writeln!(receiver)?;
		Ok(())
	}
}

impl Renderable for Attachment {
	fn render<W: Write>(&self, receiver: &mut W) -> Result<()> {
		match self {
			Attachment::Image(image) => {
				// ![Alt](/path/to/img.jpg "image title")
				if let Some(alt_text) = &image.alt_text {
					writeln!(receiver, "![{alt_text}]({})", image.file_url)?;
				} else {
					writeln!(receiver, "![]({})", image.file_url)?;
				}
				writeln!(receiver)?;
			}
			Attachment::Audio(audio) => {
				writeln!(receiver, "<figure>")?;
				writeln!(receiver, "\t<audio controls src=\"{}\"></audio>", audio.file_url)?;
				writeln!(receiver, "\t<figcaption>")?;
				writeln!(receiver, "\t\t<span>{}</span>", audio.title)?;
				writeln!(receiver, "\t\t<small>{}</small>", audio.artist)?;
				writeln!(receiver, "\t</figcaption>")?;
				writeln!(receiver, "\t<a href=\"{}\">Direct Link</a>", audio.file_url)?;
				writeln!(receiver, "</figure>")?;
				writeln!(receiver)?;
			}
		}
		Ok(())
	}
}

impl Renderable for Ask {
	fn render<W: Write>(&self, receiver: &mut W) -> Result<()> {
		write!(receiver, ">[!cohost-ask] ")?;

		if self.anon {
			write!(receiver, "**Anonymous User** asked:")?;
		} else if let Some(asking_project) = &self.asking_project {
			write!(receiver, "[**@{0}**](https://cohost.org/{0}) asked:", asking_project.handle)?;
		}

		let ask_date = self.sent_at.with_timezone(&Local);
		writeln!(
			receiver,
			" <time datetime=\"{}\">{}</time>",
			ask_date.to_rfc3339(),
			ask_date.format_localized("%a, %b %d, %Y at %H:%M", Locale::en_US),
		)?;

		writeln!(receiver, ">{}", self.content)?;
		writeln!(receiver)?;

		Ok(())
	}
}

impl Renderable for Block {
	fn render<W: Write>(&self, receiver: &mut W) -> Result<()> {
		match self {
			Block::Markdown { markdown } => markdown.render(receiver),
			Block::AttachmentRow { attachments } => {
				for attachment in attachments {
					attachment.render(receiver)?;
				}
				Ok(())
			}
			Block::Attachment { attachment } => attachment.render(receiver),
			Block::Ask { ask } => ask.render(receiver),
		}
	}
}

impl Renderable for Chost {
	fn render<W: Write>(&self, receiver: &mut W) -> Result<()> {
		let callout_post = self.cohost_post(false).unwrap();
		writeln!(receiver, "{}", callout_post.trim())?;
		writeln!(receiver)?;

		if !self.headline.is_empty() {
			writeln!(receiver, "# {}", self.headline)?;
			writeln!(receiver)?;
		}

		for block in &self.blocks {
			block.render(receiver)?;
		}

		let tags_string = &self.tags.iter().map(|tag| {
			return format!("#{tag}")
		}).collect::<Vec<String>>().join(" ");

		writeln!(receiver, "{tags_string}")?;
		writeln!(receiver)?;

		Ok(())
	}
}

impl Chost {
	pub(crate) fn is_share(&self) -> bool {
		self.transparent_share_of_post_id.or_else(|| self.share_of_post_id).is_some()
	}

	pub(crate) fn shared_chost(&self) -> Option<&Chost> {
		if !self.is_share() {
			return None;
		}

		let share_id = self.transparent_share_of_post_id.or_else(|| self.share_of_post_id);
		if share_id.is_none() {
			return None;
		}

		let share_id = share_id.unwrap();
		self.share_tree.iter().filter(|c| c.post_id == share_id).next()
	}

	/// Generates a `[!cohost-post]` callout. Generates a `[!cohost-share]` callout instead if `share` is `true`.
	/// Can be unwrapped safely if `share` is `false`.
	fn cohost_post(&self, share: bool) -> Option<String> {
		let mut markdown = String::new();

		let publish_date = self.published_at.with_timezone(&Local);

		let callout = if share { "share" } else { "post" };
		writeln!(markdown, ">[!cohost-{callout}] {}", self.posting_project.display_name).unwrap();
		write!(
			markdown,
			">@{} <time datetime=\"{}\">{}</time>",
			self.posting_project.handle,
			publish_date.to_rfc3339(),
			publish_date.format_localized("%a, %b %d, %Y at %H:%M", Locale::en_US),
		).unwrap();

		// ugly double indentation because we can't combine "if let"s and booleans yet
		if share {
			if let Some(shared_chost) = self.shared_chost() {
				write!(
					markdown,
					" :LiRefreshCw: **{}** @{}",
					shared_chost.posting_project.display_name,
					shared_chost.posting_project.handle,
				).unwrap();
			} else {
				return None;
			}
		}

		writeln!(markdown).unwrap();

		Some(markdown)
	}

	#[allow(dead_code)]
	pub(crate) fn generate_markdown(&self) -> Result<String> {
		let mut md = String::new();

		// region write obsidian front matter

		let mut users: IndexSet<String> = indexset! { self.posting_project.handle.clone() };
		let mut tags: IndexSet<String> = IndexSet::new();
		for tag in self.tags.clone() {
			tags.insert(tag);
		}

		for shared_chost in &self.share_tree {
			for block in &shared_chost.blocks {
				match block {
					Block::Ask { ask } => {
						if ask.anon {
							users.insert("Anonymous User".parse()?);
						} else {
							users.insert(ask.clone().asking_project.unwrap().handle);
						}
					}
					_ => ()
				}
			}

			users.insert(shared_chost.clone().posting_project.handle);
			for tag in shared_chost.tags.clone() {
				tags.insert(tag);
			}
		}

		// write the actual front matter
		writeln!(md, "---")?;
		writeln!(md, "date: {}", self.published_at.with_timezone(&Local).to_rfc3339())?;
		writeln!(md, "cohost/users:")?;

		for user in users {
			writeln!(md, "  - {user}")?;
		}

		if !tags.is_empty() {
			writeln!(md, "cohost/tags:")?;
			for tag in tags {
				writeln!(md, "  - {tag}")?;
			}
		}

		writeln!(md, "cohost/original-post: {}", &self.single_post_page_url)?;
		writeln!(md, "cohost/archived-post: https://web.archive.org/web/*/{}", &self.single_post_page_url)?;

		writeln!(md, "---")?;

		// endregion

		// region write posts

		if let Some(callout_share) = self.cohost_post(true) {
			writeln!(md, "{}", callout_share.trim()).unwrap();
			writeln!(md)?;
		}

		for tree_chost in self.share_tree.iter().filter(|c| c.transparent_share_of_post_id.is_none()) {
			tree_chost.render(&mut md)?;
		}

		self.render(&mut md)?;

		// endregion

		Ok(md)
	}
}