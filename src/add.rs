use std::path::Path;

use anyhow::Result;
use clap::Parser;

use crate::Note;

/// Add a note.
#[derive(Parser)]
pub struct AddCmd {
	/// The title of the entry
	title: String,
	/// The contents of the entry
	content: String,
}

impl AddCmd {
	pub fn run(self, mut notes: Vec<Note>, path: &Path) -> Result<()> {
		notes.insert(
			0,
			Note {
				title: self.title,
				content: self.content,
			},
		);

		crate::write(path, &notes)?;

		Ok(())
	}
}
