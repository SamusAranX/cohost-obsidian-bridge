use serde::Deserialize;
use serde_aux::prelude::deserialize_option_number_from_string;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AstSpan {
	pub start_index: u64,
	pub end_index: u64,
	pub ast: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AstMap {
	pub spans: Vec<AstSpan>,
	pub read_more_index: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AskSettings {
	pub enabled: bool,
	pub allow_anon: bool,
	pub require_logged_in_anon: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Privacy {
	// Private,
	Public,
	LoggedIn,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContactCardEntry {
	pub value: String,
	pub service: String,
	pub visibility: Privacy,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostingProject {
	pub handle: String,
	pub display_name: String,
	pub dek: String,
	pub description: String,
	#[serde(rename = "avatarURL")]
	pub avatar_url: String,
	#[serde(rename = "avatarPreviewURL")]
	pub avatar_preview_url: String,
	#[serde(rename = "headerURL")]
	pub header_url: Option<String>,
	#[serde(rename = "headerPreviewURL")]
	pub header_preview_url: Option<String>,
	pub project_id: u64,
	pub privacy: Privacy,
	pub pronouns: String,
	pub url: String,
	pub flags: Vec<String>,
	pub avatar_shape: String,
	pub logged_out_post_visibility: String,
	pub ask_settings: AskSettings,
	pub frequently_used_tags: Vec<String>,
	pub contact_card: Vec<ContactCardEntry>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Markdown {
	pub content: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImageAttachment {
	#[serde(rename = "fileURL")]
	pub file_url: String,
	#[serde(rename = "previewURL")]
	pub preview_url: String,
	pub attachment_id: String,
	pub alt_text: Option<String>,
	pub width: i64,
	pub height: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AudioAttachment {
	#[serde(rename = "fileURL")]
	pub file_url: String,
	#[serde(rename = "previewURL")]
	pub preview_url: String,
	pub attachment_id: String,
	pub artist: String,
	pub title: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub(crate) enum Attachment {
	Image(ImageAttachment),
	Audio(AudioAttachment),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AskingProject {
	pub project_id: i64,
	#[serde(rename = "avatarPreviewURL")]
	pub avatar_preview_url: String,
	pub avatar_shape: String,
	#[serde(rename = "avatarURL")]
	pub avatar_url: String,
	pub flags: Vec<String>,
	pub handle: String,
	pub privacy: Privacy,
	pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Ask {
	pub ask_id: String,
	pub anon: bool,
	pub logged_in: bool,
	pub asking_project: Option<AskingProject>,
	pub content: String,
	pub sent_at: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub(crate) enum Block {
	Markdown { markdown: Markdown },
	AttachmentRow { attachments: Vec<Block> },
	Attachment { attachment: Attachment },
	Ask { ask: Ask },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Chost {
	pub post_id: u64,
	pub headline: String,
	pub published_at: String,
	pub filename: String,
	pub transparent_share_of_post_id: Option<u64>,
	pub share_of_post_id: Option<u64>,
	pub state: u64,
	pub num_comments: u64,
	pub num_shared_comments: u64,
	pub cws: Vec<String>,
	pub tags: Vec<String>,
	pub blocks: Vec<Block>,
	pub plain_text_body: String,
	pub posting_project: PostingProject,
	pub share_tree: Vec<Chost>,
	pub related_projects: Vec<PostingProject>,
	pub single_post_page_url: String,
	pub effective_adult_content: bool,
	pub is_editor: bool,
	pub contributor_block_incoming_or_outgoing: bool,
	pub has_any_contributor_muted: bool,
	pub post_edit_url: String,
	pub is_liked: bool,
	pub can_share: bool,
	pub can_publish: bool,
	pub has_cohost_plus: bool,
	pub pinned: bool,
	pub comments_locked: bool,
	pub shares_locked: bool,
	pub limited_visibility_reason: String,
	pub ast_map: AstMap,
	#[serde(deserialize_with = "deserialize_option_number_from_string")]
	pub response_to_ask_id: Option<u64>,
}