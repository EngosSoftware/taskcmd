//! # TO-DO
//! - Add `lib.rs` with a function that handles everything.
//! - Add columnwise formatting when listing available tasks.
//! - Add coloring when displaying available tasks.
//! - Add colored error display.
//! - Add listing shell commands after interpolation for a task (or even all tasks).
//! - Display error when no Taskfile.idml found in the directory where taskcmd was executed.
//!
//!

use taskcmd::errors::Result;
use taskcmd::run;

fn main() -> Result<()> {
  run()
}
