use std::path::Path;

use anyhow::Result;
use clap::Parser;

use crate::Note;

/// Delete notes
#[derive(Parser)]
pub struct DeleteCmd {
	/// The index or title of notes to delete, or a range such as start..end
	#[arg(group = "identifier")]
	note: Vec<String>,
}

impl DeleteCmd {
	pub fn run(self, notes: Vec<Note>, path: &Path) -> Result<()> {
		let selectors = self
			.note
			.into_iter()
			.map(|s| {
				if let Ok(id) = s.parse::<usize>() {
					return (s, Some(id..id + 1));
				}

				// Is it a range?
				if let Some((start, end)) = s.split_once("..") {
					let start = if start.is_empty() {
						Some(0)
					} else {
						start.parse::<usize>().ok()
					};

					let end = if end.is_empty() {
						Some(notes.len())
					} else {
						end.parse::<usize>().ok()
					};

					if let (Some(start), Some(end)) = (start, end) {
						return (s, Some(start..end.saturating_add(1)));
					}
				}

				(s.to_lowercase(), None)
			})
			.collect::<Vec<_>>();

		let mut deleted = Vec::new();
		let mut remaining = Vec::new();
		for (i, n) in notes.into_iter().enumerate() {
			if selectors.iter().any(|(title, range)| {
				title == &n.title.to_lowercase()
					|| range.as_ref().is_some_and(|range| range.contains(&i))
			}) {
				deleted.push(n);
			} else {
				remaining.push(n);
			}
		}

		if deleted.is_empty() {
			eprintln!("Nothing matched the pattern");
			return Ok(());
		}

		crate::write(path, &remaining)?;

		if deleted.len() == 1 {
			println!("Deleted {}", &deleted[0].title);
		} else if deleted.len() <= 5 {
			println!("Deleted:");
			for n in &deleted {
				println!("{}", n.title);
			}
		} else {
			println!("Deleted {} notes", deleted.len());
		}

		Ok(())
	}
}
