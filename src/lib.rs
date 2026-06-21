mod command;
mod errors;
mod loader;
mod tasks;

use crate::loader::load_definitions;
use crate::tasks::Tasks;
pub use errors::*;

pub fn run() -> Result<()> {
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
