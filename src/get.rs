use std::process;

use anyhow::Result;
use clap::Parser;

use crate::Note;

/// Print the contents of a note
#[derive(Parser)]
pub struct GetCmd {
	// Title or index of a note.
	note: String,
}

impl GetCmd {
	pub fn run(self, notes: Vec<Note>) -> Result<()> {
		let note = self
			.note
			.parse::<usize>()
			.ok()
			.and_then(|i| notes.get(i))
			.or_else(|| {
				let title = self.note.to_lowercase();
				notes.iter().find(|n| title == n.title.to_lowercase())
			});

		match note {
			None => {
				eprintln!("No note matched the provided title / index");
				process::exit(1);
			}
			Some(n) => {
				println!("{}", n.content);
				Ok(())
			}
		}
	}
}
