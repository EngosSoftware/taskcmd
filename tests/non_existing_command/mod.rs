#[test]
fn _0001() {
  cli_assert::command!()
    .arg("run")
    .failure()
    .code(127)
    .stdout("")
    .stderr("zsh:1: command not found: copacabana\n")
    .execute();
}

#[test]
fn _0002() {
  cli_assert::command!()
    .arg("build")
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: Task not found: build\n")
    .execute();
}
