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
