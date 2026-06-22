//! # TO-DO
//! - Add `lib.rs` with a function that handles everything.
//! - Add columnwise formatting when listing available tasks.
//! - Add coloring when displaying available tasks.
//! - Add colored error display.
//! - Add listing shell commands after interpolation for a task (or even all tasks).
//! - Display error when no Taskfile.idml found in the directory where taskcmd was executed.
//!
//!

use antex::{ColorMode, StyledText, Text};
use taskcmd::run;

fn main() {
  let cm = ColorMode::default();
  match run() {
    Ok(()) => {}
    Err(err) => {
      eprintln!("{}: {}", Text::new(cm).red().bold().s("error").reset(), err.as_text(cm));
      std::process::exit(1);
    }
  }
}
