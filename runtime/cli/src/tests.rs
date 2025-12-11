use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn script() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd
        .arg(format!("{}/fixtures/script.santa", env!("CARGO_MANIFEST_DIR")))
        .assert();
    assert.success().stdout("14\n");
}

#[test]
fn solution() {
    #[allow(deprecated)]
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
    #[allow(deprecated)]
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
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("-r").write_stdin("[1, 2] + [3]").assert();
    assert.success().stdout(predicate::str::contains("[1, 2, 3]"));
}

#[test]
fn eval_simple_expression() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("-e").arg("1 + 2").assert();
    assert.success().stdout("3\n");
}

#[test]
fn eval_complex_expression() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("-e").arg("map(|x| x * 2, [1, 2, 3])").assert();
    assert.success().stdout("[2, 4, 6]\n");
}

#[test]
fn eval_aoc_solution() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("-e").arg("part_one: { 42 }").assert();
    assert
        .success()
        .stdout(predicate::str::contains("Part 1: \u{1b}[32m42\u{1b}[0m"));
}

#[test]
fn stdin_simple_expression() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.write_stdin("1 + 2").assert();
    assert.success().stdout("3\n");
}

#[test]
fn stdin_aoc_solution() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.write_stdin("part_one: { 42 }").assert();
    assert
        .success()
        .stdout(predicate::str::contains("Part 1: \u{1b}[32m42\u{1b}[0m"));
}

#[test]
fn stdin_empty() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.write_stdin("").assert();
    // Empty program should succeed with no output
    assert.success();
}

#[test]
fn fmt_stdout_simple_expression() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("-f").arg("-e").arg("1+2").assert();
    assert.success().stdout("1 + 2\n");
}

#[test]
fn fmt_stdout_list() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("-f").arg("-e").arg("[1,2,3]").assert();
    assert.success().stdout("[1, 2, 3]\n");
}

#[test]
fn fmt_stdout_lambda() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("-f").arg("-e").arg("|x|x+1").assert();
    assert.success().stdout("|x| x + 1\n");
}

#[test]
fn fmt_stdout_let() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("-f").arg("-e").arg("let x=1").assert();
    assert.success().stdout("let x = 1\n");
}

#[test]
fn fmt_stdout_string_with_escapes() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("-f").arg("-e").arg(r#""hello\nworld""#).assert();
    assert.success().stdout("\"hello\\nworld\"\n");
}

#[test]
fn fmt_stdin() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("-f").write_stdin("1+2").assert();
    assert.success().stdout("1 + 2\n");
}

#[test]
fn fmt_check_passes_for_formatted() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("--fmt-check").arg("-e").arg("1 + 2\n").assert();
    assert.success();
}

#[test]
fn fmt_check_fails_for_unformatted() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("--fmt-check").arg("-e").arg("1+2").assert();
    assert.code(1);
}

#[test]
fn fmt_write_requires_file() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("--fmt-write").arg("-e").arg("1+2").assert();
    assert.code(1).stderr(predicate::str::contains("requires a file path"));
}

#[test]
fn fmt_invalid_syntax_returns_error() {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("santa-cli").unwrap();
    let assert = cmd.arg("-f").arg("-e").arg("let = ").assert();
    assert.code(2).stderr(predicate::str::contains("Parse error"));
}
