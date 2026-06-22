//! # Definition of result and errors

use antex::{ColorMode, StyledText, Text};

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
}

impl TaskCmdError {
  /// Returns a colored error message.
  pub fn as_text(&self, cm: ColorMode) -> Text {
    match self {
      TaskCmdError::SpawnCommandFailed(reason) => Text::new(cm).s("failed to spawn command, reason: ").cyan().s(reason).reset(),
      TaskCmdError::DuplicatedTask(name) => Text::new(cm).s("duplicated task: ").cyan().s(name).reset(),
      TaskCmdError::TaskNotFound(name) => Text::new(cm).s("task not found: ").cyan().s(name).reset(),
      TaskCmdError::DependencyCycle(name) => Text::new(cm).s("dependency cycle for task: ").cyan().s(name).reset(),
      TaskCmdError::UnexpectedNode(name) => Text::new(cm).s("unexpected node: ").cyan().s(name).reset(),
      TaskCmdError::ZeroOrOneAttributeAllowed(name) => Text::new(cm).s("at most one attribute '").cyan().s(name).reset().s("' allowed"),
      TaskCmdError::MissingDefinitions => Text::new(cm).s("task definitions file is missing or inaccessible"),
    }
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
