#[test]
fn _0001() {
  cli_assert::command!().arg("hello").success().code(0).stdout("Hello!\n").stderr("").execute();
}
