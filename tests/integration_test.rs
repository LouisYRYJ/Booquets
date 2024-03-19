use std::process::{Command, Output};

#[test]
fn test_cli() {
    let output = run_cli_command();
    assert!(output.status.success());
    let result = String::from_utf8(output.stdout).expect("stdout is not valid");
    println!("{}", result);
}

fn run_cli_command() -> Output {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "A * (B + C)",
            "tests/test_documents/test_A.txt",
        ])
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    output
}
