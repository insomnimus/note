use anyhow::Result;
use clap::Parser;

use crate::Note;

/// List notes
#[derive(Parser)]
pub struct ListCmd {
	/// List notes starting from an index
	#[arg(short, long, default_value_t = 0)]
	pub start: usize,

	/// List notes up to an index
	#[arg(short, long, default_value_t = usize::MAX, hide_default_value = true)]
	pub end: usize,
}

impl ListCmd {
	pub fn run(self, notes: Vec<Note>) -> Result<()> {
		let max_title = notes
			.iter()
			.skip(self.start)
			.take(self.end.saturating_sub(self.start))
			.map(|n| n.title.len())
			.max()
			.unwrap_or(0);
		let max_digits = usize::min(self.end, notes.len())
			.checked_ilog10()
			.unwrap_or(0) as usize;

		for (i, n) in notes
			.iter()
			.enumerate()
			.skip(self.start)
			.take(self.end.saturating_sub(self.start))
		{
			println!(
				"#{i:<max_digits$} {title:<max_title$} | {content}",
				title = n.title,
				content = n.content,
			);
		}

		Ok(())
	}
}
