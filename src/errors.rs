//! # Definition of result and errors

/// Common result type.
pub type Result<T, E = TaskCmdError> = std::result::Result<T, E>;

/// Errors.
#[derive(PartialEq, Eq)]
pub enum TaskCmdError {
  SpawnCommandFailed(String),
  DuplicatedTask(String),
  TaskNotFound(String),
  DependencyCycle(String),
  UnexpectedNode(String),
  ZeroOrOneAttributeAllowed(String),
  MissingDefinitions,
  MultipleDefinitions(String),
}

impl std::fmt::Display for TaskCmdError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        TaskCmdError::SpawnCommandFailed(reason) => format!("Failed to spawn command, reason: {}", reason),
        TaskCmdError::DuplicatedTask(name) => format!("Duplicated task: {}", name),
        TaskCmdError::TaskNotFound(name) => format!("Task not found: {}", name),
        TaskCmdError::DependencyCycle(name) => format!("Dependency cycle for task: {}", name),
        TaskCmdError::UnexpectedNode(name) => format!("Unexpected node: {}", name),
        TaskCmdError::ZeroOrOneAttributeAllowed(name) => format!("At most one attribute '{}' allowed", name),
        TaskCmdError::MissingDefinitions => "Task definitions file is missing or inaccessible".to_string(),
        TaskCmdError::MultipleDefinitions(files) => format!("Multiple task definitions files found: {}", files),
      }
    )
  }
}

impl std::fmt::Debug for TaskCmdError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}

pub fn err_command_spawn(reason: String) -> TaskCmdError {
  TaskCmdError::SpawnCommandFailed(reason)
}

pub fn err_duplicated_task(task_name: String) -> TaskCmdError {
  TaskCmdError::DuplicatedTask(task_name)
}

pub fn err_task_not_found(name: String) -> TaskCmdError {
  TaskCmdError::TaskNotFound(name)
}

pub fn err_dependency_cycle(name: String) -> TaskCmdError {
  TaskCmdError::DependencyCycle(name)
}

pub fn err_unexpected_node(name: String) -> TaskCmdError {
  TaskCmdError::UnexpectedNode(name)
}

pub fn err_zero_or_one_attribute_allowed(name: String) -> TaskCmdError {
  TaskCmdError::ZeroOrOneAttributeAllowed(name)
}

pub fn err_definitions_not_found() -> TaskCmdError {
  TaskCmdError::MissingDefinitions
}

pub fn err_multiple_definitions(files: String) -> TaskCmdError {
  TaskCmdError::MultipleDefinitions(files)
}
