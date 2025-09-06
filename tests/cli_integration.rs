use std::process::Command;

#[test]
fn test_cli_single_reference() {
    let output = Command::new("cargo")
        .args(["run", "--", "--reference", "Genesis 1:1"])
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
        .args(["run", "--", "--text", "See Genesis 1:1 and 2 Nephi 10:14"])
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
        .args(["run", "--", "--reference", "InvalidBook 1:1"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Unknown book abbreviation: 'InvalidBook'"));
}

#[test]
fn test_cli_invalid_chapter() {
    let output = Command::new("cargo")
        .args(["run", "--", "--reference", "Genesis 999:1"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Chapter 999 does not exist in Genesis"));
}

#[test]
fn test_cli_invalid_verse() {
    let output = Command::new("cargo")
        .args(["run", "--", "--reference", "Genesis 1:999"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Verse 999 does not exist in Genesis 1"));
}

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Generate links to scriptures"));
    assert!(stdout.contains("--reference"));
    assert!(stdout.contains("--text"));
    assert!(stdout.contains("--file"));
}

#[test]
fn test_cli_json_output() {
    let output = Command::new("cargo")
        .args(["run", "--", "--reference", "Genesis 1:1", "--json"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Parse as JSON to ensure it's valid
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Invalid JSON output");
    assert_eq!(json["success"], true);
    assert_eq!(json["input"], "Genesis 1:1");
    assert!(
        json["url"]
            .as_str()
            .unwrap()
            .contains("churchofjesuschrist.org")
    );
}

#[test]
fn test_cli_json_error() {
    let output = Command::new("cargo")
        .args(["run", "--", "--reference", "InvalidBook 1:1", "--json"])
        .output()
        .expect("Failed to execute command");

    // In JSON mode, CLI returns success but outputs error as JSON
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Parse as JSON to ensure it's valid
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Invalid JSON output");
    assert_eq!(json["success"], false);
    assert_eq!(json["input"], "InvalidBook 1:1");
    assert!(
        json["error"]["message"]
            .as_str()
            .unwrap()
            .contains("Unknown book")
    );
}

#[test]
fn test_cli_batch_processing() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--batch",
            "Genesis 1:1,2 Nephi 10:14,InvalidRef",
            "--json",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Parse as JSON to ensure it's valid
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Invalid JSON output");
    assert_eq!(json["total_processed"], 3);
    assert_eq!(json["successful"], 2);
    assert_eq!(json["failed"], 1);
    assert_eq!(json["results"].as_array().unwrap().len(), 3);
}

#[test]
fn test_cli_validate_only() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--reference",
            "Genesis 1:1",
            "--validate-only",
            "--json",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Parse as JSON to ensure it's valid
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Invalid JSON output");
    assert_eq!(json["success"], true);
    assert_eq!(json["input"], "Genesis 1:1");
    // URL should be null in validate-only mode
    assert!(json["url"].is_null());
}

#[test]
fn test_cli_text_processing_json() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--text",
            "See Genesis 1:1 and 2 Nephi 10:14",
            "--json",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Parse as JSON to ensure it's valid
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Invalid JSON output");
    assert_eq!(json["success"], true);
    assert_eq!(json["references_found"], 2);
    assert!(
        json["output_text"]
            .as_str()
            .unwrap()
            .contains("[Genesis 1:1](")
    );
}

#[test]
fn test_cli_file_processing() {
    use std::fs;

    // Create a temporary file with scripture references
    let test_content = "See Genesis 1:1 and 2 Nephi 10:14 for insights.";
    fs::write("test_file.txt", test_content).expect("Failed to write test file");

    let output = Command::new("cargo")
        .args(["run", "--", "--file", "test_file.txt"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("[Genesis 1:1]("));
    assert!(stdout.contains("[2 Nephi 10:14]("));

    // Clean up
    fs::remove_file("test_file.txt").ok();
}

#[test]
fn test_cli_file_processing_json() {
    use std::fs;

    // Create a temporary file with scripture references
    let test_content = "See Genesis 1:1 for creation.";
    fs::write("test_file_json.txt", test_content).expect("Failed to write test file");

    let output = Command::new("cargo")
        .args(["run", "--", "--file", "test_file_json.txt", "--json"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Parse as JSON to ensure it's valid
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Invalid JSON output");
    assert_eq!(json["success"], true);
    assert_eq!(json["references_found"], 1);

    // Clean up
    fs::remove_file("test_file_json.txt").ok();
}

#[test]
fn test_cli_file_not_found() {
    let output = Command::new("cargo")
        .args(["run", "--", "--file", "nonexistent_file.txt"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Error reading file"));
}

#[test]
fn test_cli_file_not_found_json() {
    let output = Command::new("cargo")
        .args(["run", "--", "--file", "nonexistent_file.txt", "--json"])
        .output()
        .expect("Failed to execute command");

    // In JSON mode, file errors return success but output error as JSON
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Parse as JSON to ensure it's valid
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Invalid JSON output");
    assert_eq!(json["success"], false);
    assert_eq!(json["references_found"], 0);
}

#[test]
fn test_cli_no_arguments() {
    let output = Command::new("cargo")
        .args(["run", "--"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Please provide either --reference, --batch, --text, or --file"));
}

#[test]
fn test_cli_validate_only_non_json() {
    let output = Command::new("cargo")
        .args(["run", "--", "--reference", "Genesis 1:1", "--validate-only"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Valid: Genesis 1:1"));
}

#[test]
fn test_cli_validate_only_error_non_json() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--reference",
            "InvalidBook 1:1",
            "--validate-only",
        ])
        .output()
        .expect("Failed to execute command");

    // Validation mode returns success but prints "Invalid:" for invalid references
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Invalid: InvalidBook 1:1"));
    assert!(stdout.contains("Unknown book"));
}

#[test]
fn test_cli_validate_only_error_json() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--reference",
            "InvalidBook 1:1",
            "--validate-only",
            "--json",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Parse as JSON to ensure it's valid
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Invalid JSON output");
    assert_eq!(json["success"], false);
    assert_eq!(json["valid"], false);
    assert_eq!(json["input"], "InvalidBook 1:1");
    assert!(
        json["error"]["message"]
            .as_str()
            .unwrap()
            .contains("Unknown book")
    );
}
