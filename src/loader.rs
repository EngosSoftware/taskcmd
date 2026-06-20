//! # Task definitions loader

use super::errors::*;
use std::fs::{read_dir, read_to_string};

/// Returns the content of the task definitions file.
///
/// Error handling is intentionally minimal. File access and filesystem errors
/// are not reported precisely because such failures are expected to be rare.
/// This function will just panic if the task definitions file is not accessible.
pub fn load_definitions() -> Result<String> {
  let mut files = vec![];
  let paths = read_dir(".").expect("Failed to access current directory");
  for entry in paths.flatten() {
    let file_type = entry.file_type().expect("Failed to get file type");
    if file_type.is_file() {
      let file_name = entry.file_name();
      if file_name.eq_ignore_ascii_case("taskcmd.idml") {
        files.push(file_name.to_string_lossy().to_string());
      }
    }
  }
  match files.len() {
    0 => Err(err_definitions_not_found()),
    1 => Ok(read_to_string(&files[0]).expect("Failed to read task definitions file")),
    _ => Err(err_multiple_definitions(files.join(", "))),
  }
}
