#![doc = include_str!("../docs/README.md")]

pub mod command;
pub mod errors;
pub mod loader;
pub mod tasks;

use crate::errors::*;
use crate::loader::load_definitions;
use crate::tasks::Tasks;
use antex::ColorMode;

pub fn run() -> Result<()> {
  let definitions = load_definitions()?;
  let tasks = Tasks::new(definitions)?;
  let args = std::env::args().skip(1).collect::<Vec<String>>();
  if args.len() == 1 {
    let task_name = &args[0];
    let exit_status = tasks.execute(task_name)?;
    std::process::exit(exit_status.code().unwrap());
  } else {
    tasks.list(ColorMode::default());
  }
  Ok(())
}
