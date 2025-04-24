use std::process;

use anyhow::{
	anyhow,
	Result,
};
use clap::Parser;
use url::Url;

use crate::Note;

/// Open a note with a URL in your browser.
#[derive(Parser)]
pub struct OpenCmd {
	/// Title or index of the note
	note: String,
}

impl OpenCmd {
	pub fn run(self, notes: Vec<Note>) -> Result<()> {
		let title = self.note.to_lowercase();
		let note = self
			.note
			.parse::<usize>()
			.ok()
			.and_then(|i| notes.get(i))
			.or_else(|| notes.iter().find(|n| n.title.to_lowercase() == title));

		match note {
			None => {
				eprintln!("No note matched the given title / index");
				process::exit(1);
			}
			Some(n) => {
				// If it's not a URL, print something informative.
				if Url::parse(&n.content).is_err() {
					return Err(anyhow!("The content for `{}` is not a URL", n.title));
				}

				::open::that(&n.content)?;
				Ok(())
			}
		}
	}
}
