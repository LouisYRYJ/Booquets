use std::process::{Command, Output};

#[test]
fn test_cli_a() {
    let output = run_cli_command("tests/test_documents/test_A.txt");
    assert!(output.status.success());
    let result = String::from_utf8(output.stdout).expect("stdout is not valid");
    assert_eq!(result, "This document fulfills the query: false\n");
}
#[test]
fn test_cli_ab() {
    let output = run_cli_command("tests/test_documents/test_AB.txt");
    assert!(output.status.success());
    let result = String::from_utf8(output.stdout).expect("stdout is not valid");
    assert_eq!(result, "This document fulfills the query: true\n");
}
#[test]
fn test_cli_c() {
    let output = run_cli_command("tests/test_documents/test_C.txt");
    assert!(output.status.success());
    let result = String::from_utf8(output.stdout).expect("stdout is not valid");
    assert_eq!(result, "This document fulfills the query: false\n");
}

fn run_cli_command(test_file: &str) -> Output {
    let output = Command::new("cargo")
        .args(&["run", "--", "A * (B + C)", test_file])
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    output
}
