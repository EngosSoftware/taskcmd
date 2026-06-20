#[test]
fn _0001() {
  cli_assert::command!().arg("greeting").success().code(0).stdout("Hello world!\n").stderr("").execute();
}
