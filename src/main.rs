mod add;
mod delete;
mod get;
mod list;
mod open;

use std::{
	fs,
	io::ErrorKind,
	path::{
		Path,
		PathBuf,
	},
	process,
};

use anyhow::{
	anyhow,
	Result,
};
use clap::Parser;
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Serialize, Deserialize, Clone)]
struct Note {
	title: String,
	content: String,
}

fn write(path: &Path, notes: &[Note]) -> Result<()> {
	let s = serde_json::to_string_pretty(notes)?;
	fs::write(path, s.as_bytes())?;
	Ok(())
}

/// Manage to-do's from the commandline
#[derive(Parser)]
#[command(version)]
struct App {
	/// Path to the notes JSON file
	#[arg(long, env = "NOTE_NOTES_PATH")]
	notes_path: PathBuf,

	#[command(subcommand)]
	cmd: Option<Cmd>,
}

#[derive(Parser)]
enum Cmd {
	Add(add::AddCmd),
	Delete(delete::DeleteCmd),
	Get(get::GetCmd),
	List(list::ListCmd),
	Open(open::OpenCmd),
}

fn main() {
	fn inner() -> Result<()> {
		let app = App::parse();

		let notes: Vec<Note> = match fs::read_to_string(&app.notes_path) {
			Err(e) if e.kind() == ErrorKind::NotFound => Vec::new(),
			Err(e) => {
				return Err(anyhow!(
					"failed to read the notes file at {}: {}",
					app.notes_path.display(),
					e
				))
			}
			Ok(data) => serde_json::from_str(&data).map_err(|e| {
				anyhow!(
					"failed to parse the notes file at {}: {}",
					app.notes_path.display(),
					e
				)
			})?,
		};

		let cmd = app
			.cmd
			.unwrap_or(Cmd::List(list::ListCmd { start: 0, end: 5 }));

		match cmd {
			Cmd::Add(x) => x.run(notes, &app.notes_path),
			Cmd::Delete(x) => x.run(notes, &app.notes_path),
			Cmd::Get(x) => x.run(notes),
			Cmd::List(x) => x.run(notes),
			Cmd::Open(x) => x.run(notes),
		}
	}

	if let Err(e) = inner() {
		eprintln!("error: {e}");
		process::exit(1);
	}
}
