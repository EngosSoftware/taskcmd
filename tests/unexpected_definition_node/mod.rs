#[test]
fn _0001() {
  cli_assert::command!()
    .failure()
    .code(1)
    .stdout("")
    .stderr("Error: Unexpected node: description\n")
    .execute();
}
