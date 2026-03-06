use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_cli_accepts_single_expression_from_stdin() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_pauli_repl"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("binary should launch");

    child
        .stdin
        .as_mut()
        .expect("stdin should be available")
        .write_all(b"X*Y\nexit\n")
        .expect("stdin write should succeed");

    let output = child
        .wait_with_output()
        .expect("binary should exit cleanly");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("result: iZ"));
}
