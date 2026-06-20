#[test]
fn _0001() {
  cli_assert::command!().arg("parent").success().code(0).stdout("Hello parent!\n").stderr("").execute();
}
