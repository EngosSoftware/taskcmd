#[test]
fn _0001() {
  cli_assert::command!()
    .arg("aunt")
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: Dependency cycle for task: uncle\n")
    .execute();
}
