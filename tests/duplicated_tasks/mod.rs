#[test]
fn _0001() {
  cli_assert::command!()
    .arg("build")
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: Duplicated task: build\n")
    .execute();
}
