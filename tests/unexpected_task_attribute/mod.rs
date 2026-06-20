#[test]
fn _0001() {
  cli_assert::command!()
    .arg("hello")
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: Unexpected node: parallel\n")
    .execute();
}
