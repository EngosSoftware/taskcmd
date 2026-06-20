//! # TO-DO
//! - Add `lib.rs` with a function that handles everything.
//! - Add columnwise formatting when listing available tasks.
//! - Add coloring when displaying available tasks.
//! - Add colored error display.
//! - Add listing shell commands after interpolation for a task (or even all tasks).
//! - Display error when no Taskfile.idml found in the directory where taskcmd was executed.
//!
//!

mod command;
mod errors;
mod loader;
mod tasks;

use crate::loader::load_definitions;
use crate::tasks::Tasks;
use errors::*;

fn main() -> Result<()> {
  let definitions = load_definitions()?;
  let tasks = Tasks::new(definitions)?;
  let args = std::env::args().skip(1).collect::<Vec<String>>();
  if args.len() == 1 {
    let task_name = &args[0];
    let exit_status = tasks.execute(task_name)?;
    std::process::exit(exit_status.code().unwrap());
  } else {
    println!("Available tasks:");
    for (_, task) in &tasks {
      print!("  {}", task.get_name());
      if let Some(description) = task.get_description() {
        print!(" {}", description);
      }
      println!();
    }
  }
  Ok(())
}
