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
fn test_cli_dc_normalized() {
    // DC (no ampersand) should be accepted and output as D&C in links
    let output = Command::new("cargo")
        .args(["run", "--", "--reference", "DC 88:1"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("churchofjesuschrist.org/study/scriptures/dc-testament/dc/88"));

    let output_text = Command::new("cargo")
        .args(["run", "--", "--text", "Read DC 121:41 today"])
        .output()
        .expect("Failed to execute command");

    assert!(output_text.status.success());
    let stdout_text = String::from_utf8(output_text.stdout).unwrap();
    assert!(stdout_text.contains("[D&C 121:41]("));
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
fn test_cli_in_place_modifies_file() {
    use std::fs;
    use tempfile::NamedTempFile;

    let temp = NamedTempFile::new().expect("create temp file");
    let path = temp.path().to_str().expect("path to str").to_string();
    fs::write(&path, "See Alma 5:6 for a verse.").expect("write");

    let output = Command::new("cargo")
        .args(["run", "--", "--file", &path, "--in-place"])
        .output()
        .expect("run CLI");

    assert!(output.status.success());
    let content = fs::read_to_string(&path).expect("read back");
    assert!(content.contains("[Alma 5:6]("));
    assert!(content.contains("churchofjesuschrist.org"));
}

#[test]
fn test_cli_in_place_no_change_when_no_refs() {
    use std::fs;
    use tempfile::NamedTempFile;

    let temp = NamedTempFile::new().expect("create temp file");
    let path = temp.path().to_str().expect("path to str").to_string();
    let original = "No scripture refs here.";
    fs::write(&path, original).expect("write");

    let output = Command::new("cargo")
        .args(["run", "--", "--file", &path, "--in-place"])
        .output()
        .expect("run CLI");

    assert!(output.status.success());
    let content = fs::read_to_string(&path).expect("read back");
    assert_eq!(content, original);
}

#[test]
fn test_cli_format_wikilink() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--reference",
            "Alma 13:6",
            "--format",
            "wikilink",
        ])
        .output()
        .expect("run CLI");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "[[Alma 13]]:6");

    let output_text = Command::new("cargo")
        .args([
            "run",
            "--",
            "--text",
            "Read Alma 13:6 and Moroni 7:45-48.",
            "--format",
            "wikilink",
        ])
        .output()
        .expect("run CLI");

    assert!(output_text.status.success());
    let stdout_text = String::from_utf8(output_text.stdout).unwrap();
    assert!(stdout_text.contains("[[Alma 13]]:6"));
    assert!(stdout_text.contains("[[Moroni 7]]:45-48"));
}

#[test]
fn test_cli_in_place_requires_file() {
    let output = Command::new("cargo")
        .args(["run", "--", "--in-place"])
        .output()
        .expect("run CLI");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("--in-place can only be used with --file"));
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
fn test_cli_version_flag() {
    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("scripture-links"));
    assert!(stdout.contains("1.2.1")); // Current version
}

#[test]
fn test_cli_version_short_flag() {
    let output = Command::new("cargo")
        .args(["run", "--", "-V"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("scripture-links"));
    assert!(stdout.contains("1.2.1")); // Current version
}

#[test]
fn test_cli_help_shows_version() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("-V, --version"));
    assert!(stdout.contains("Print version"));
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
