use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn script() {
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd
        .arg(format!("{}/fixtures/script.santa", env!("CARGO_MANIFEST_DIR")))
        .assert();
    assert.success().stdout("14\n");
}

#[test]
fn solution() {
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd
        .arg(format!("{}/fixtures/solution.santa", env!("CARGO_MANIFEST_DIR")))
        .assert();
    assert
        .success()
        .stdout(predicate::str::contains("Part 1: \u{1b}[32m232\u{1b}[0m"))
        .stdout(predicate::str::contains("Part 2: \u{1b}[32m1783\u{1b}[0m"));
}

#[test]
fn test_solution() {
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd
        .arg("-t")
        .arg(format!("{}/fixtures/solution.santa", env!("CARGO_MANIFEST_DIR")))
        .assert();
    assert
        .success()
        .stdout(predicate::str::contains("Testcase #1"))
        .stdout(predicate::str::contains("Part 1: -1 \u{1b}[32m✔\u{1b}[0m"))
        .stdout(predicate::str::contains("Part 2: 5 \u{1b}[32m✔\u{1b}[0m"));
}

#[test]
fn repl() {
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("-r").write_stdin("[1, 2] + [3]").assert();
    assert.success().stdout(predicate::str::contains("[1, 2, 3]"));
}
