#[test]
fn _0001() {
  cli_assert::command!()
    .arg("parent")
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: Task not found: child\n")
    .execute();
}
