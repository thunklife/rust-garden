use assert_cmd::Command;
use assert_cmd::assert::Assert;
use color_eyre::eyre::Result;

#[test]
fn test_help() -> Result<()> {
  let mut cmd: Command = Command::cargo_bin("garden")?;
  let assert: Assert = cmd.arg("--help").assert();
  assert.success().stderr("");
  Ok(())
}