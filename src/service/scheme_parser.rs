use std::path::PathBuf;
use std::process::Command;

pub fn get_targets(workspace: &PathBuf) -> Result<Vec<String>, String> {
    /*
    let workspace_file = workspace.to_str()
        .expect("Workspace file is not valid UTF-8");
    let output = Command::new("xcodebuild")
        .args(["-list", "-workspace", workspace_file])
        .output()
        .expect("failed to execute xcodebuild");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut combined_output = String::with_capacity(stdout.len() + stderr.len());
    combined_output.push_str(&stdout);
    combined_output.push_str(&stderr);

    if !output.status.success() {
        return Err(format!(
            "Failed to load schemes from workspace {}.\n{}",
            workspace.display(),
            combined_output.trim()
        ));
    }

    let workspace_scheme_name = workspace_scheme_name(workspace);
    Ok(parse_schemes(&combined_output, &workspace_scheme_name))
     */
    let mut targets: Vec<String> = Vec::new();
    targets.push("Hll".to_string());
    targets.push("Hll".to_string());
    targets.push("Hll".to_string());
    targets.push("Hll".to_string());
    targets.push("Hll".to_string());

    Ok(targets)
}

fn parse_schemes(output: &str, workspace_scheme_name: &str) -> Vec<String> {
    let mut schemes = Vec::new();
    let mut in_section = false;

    for line in output.lines() {
        let trimmed = line.trim();

        if trimmed == "Schemes:" {
            in_section = true;
            continue;
        }

        if in_section {
            if trimmed.is_empty() {
                break;
            }
            if is_target_scheme(trimmed, workspace_scheme_name) {
                schemes.push(trimmed.to_string());
            }
        }
    }
    schemes
}

fn workspace_scheme_name(workspace: &PathBuf) -> String {
    let workspace_name = workspace
        .file_stem()
        .expect("Workspace file does not have a valid name")
        .to_str()
        .expect("Workspace file name is not valid UTF-8");

    format!("{workspace_name}-Workspace")
}

fn is_target_scheme(scheme: &str, workspace_scheme_name: &str) -> bool {
    scheme != workspace_scheme_name &&
        !scheme.contains('_') &&
        !scheme.contains("Tests")
}