// The `assert_cmd` crate is helpful in running
// the primay binary and checking it's behavior
use assert_cmd::Command;
use assert_fs::prelude::*;
// The `predicates` crate helps to write assertions
// which `assert_cmd` can test against.
use predicates::prelude::*;

type DynResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn nand_stays_nand_as_cli_arg() -> DynResult {
    let mut cmd = Command::cargo_bin("nandu")?;

    cmd.arg("Nand(a, b)");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Nand(a, b)"));
    Ok(())
}

#[test]
fn nand_stays_nand_as_pipe() -> DynResult {
    let file = assert_fs::NamedTempFile::new("nand.txt")?;
    file.write_str("Nand(a, b)")?;

    let mut cmd = Command::cargo_bin("nandu")?;
    cmd.pipe_stdin(file)?;
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Nand(a, b)"));
    Ok(())
}

#[test]
fn cli_args_overwrite_pipe_inputs() -> DynResult {
    let file = assert_fs::NamedTempFile::new("nand.txt")?;
    file.write_str("Xor(a, b)")?;

    let mut cmd = Command::cargo_bin("nandu")?;
    cmd.arg("Nand(a, b)");
    cmd.pipe_stdin(file)?;
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Nand(a, b)"));
    Ok(())
}

#[test]
fn cli_error_if_trailing_parens() -> DynResult {
    let mut cmd = Command::cargo_bin("nandu")?;
    cmd.arg("Nand(a, b))"); // Input with trailing
                            // parentheses.
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Error"))
        .stderr(predicates::str::contains("')'"));
    Ok(())
}
