use std::process::Command;

#[test]
fn test_cli_single_reference() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--reference", "Genesis 1:1"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("https://www.churchofjesuschrist.org/study/scriptures/ot/gen/1"));
    assert!(stdout.contains("lang=eng&id=p1#p1"));
}

#[test]
fn test_cli_text_processing() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--text", "See Genesis 1:1 and 2 Nephi 10:14"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("[Genesis 1:1]("));
    assert!(stdout.contains("[2 Nephi 10:14]("));
}

#[test]
fn test_cli_invalid_reference() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--reference", "InvalidBook 1:1"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Unknown book abbreviation: 'InvalidBook'"));
}

#[test]
fn test_cli_invalid_chapter() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--reference", "Genesis 999:1"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Chapter 999 does not exist in Genesis"));
}

#[test]
fn test_cli_invalid_verse() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--reference", "Genesis 1:999"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Verse 999 does not exist in Genesis 1"));
}

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Generate links to scriptures"));
    assert!(stdout.contains("--reference"));
    assert!(stdout.contains("--text"));
    assert!(stdout.contains("--file"));
}
