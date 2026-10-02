use std::process::Command;
use std::path::{Path, PathBuf};
use std::time::Instant;
use anyhow::{Result, Context};
use crate::claim::{Claim, VerificationStatus, VerificationResult, Evidence};
use chrono::Utc;

pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub repo_url: Option<String>,
    pub setup_commands: Vec<String>,
    pub verify_commands: Vec<String>,
    pub timeout_seconds: u64,
    pub expected_outcome: ExpectedOutcome,
}

pub enum ExpectedOutcome {
    Pass(String),
    ExitCode(i32),
    Both(String, i32),
}

pub struct TaskResult {
    pub task_id: String,
    pub candidate_id: String,
    pub status: VerificationStatus,
    pub evidence: Vec<Evidence>,
    pub duration_seconds: u64,
    pub checksum: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct Harness {
    sandbox_dir: PathBuf,
    timeout_seconds: u64,
}

impl Harness {
    pub fn new(sandbox_dir: &str, timeout_seconds: u64) -> Self {
        Harness {
            sandbox_dir: PathBuf::from(sandbox_dir),
            timeout_seconds,
        }
    }

    pub fn execute_task(&self, task: &Task, candidate_id: &str) -> Result<TaskResult> {
        let start = Instant::now();
        let temp_dir = tempfile::tempdir().context("Failed to create temp dir")?;
        let repo_path = self.clone_repo(&task.repo_url, temp_dir.path())?;

        let mut evidence = Vec::new();
        for cmd in &task.setup_commands {
            let result = self.run_command(cmd, &repo_path)?;
            evidence.push(result);
        }

        let mut all_passed = true;
        for cmd in &task.verify_commands {
            let result = self.run_command(cmd, &repo_path)?;
            evidence.push(result.clone());

            let passed = match &task.expected_outcome {
                ExpectedOutcome::Pass(expected) => result.output.contains(expected),
                ExpectedOutcome::ExitCode(code) => result.exit_code == *code,
                ExpectedOutcome::Both(expected, code) => {
                    result.output.contains(expected) && result.exit_code == *code
                }
            };

            if !passed {
                all_passed = false;
            }
        }

        let duration = start.elapsed().as_secs();

        Ok(TaskResult {
            task_id: task.id.clone(),
            candidate_id: candidate_id.to_string(),
            status: if all_passed { VerificationStatus::Pass } else { VerificationStatus::Fail },
            evidence,
            duration_seconds: duration,
            checksum: String::new(),
            timestamp: Utc::now(),
        })
    }

    fn run_command(&self, command: &str, cwd: &PathBuf) -> Result<Evidence> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(cwd)
            .output()
            .context("Failed to execute command")?;

        let exit_code = output.status.code().unwrap_or(-1);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Ok(Evidence {
            command: command.to_string(),
            output: format!("{}\n{}", stdout, stderr),
            exit_code,
            timestamp: Utc::now(),
        })
    }

    fn clone_repo(&self, url: &Option<String>, path: &Path) -> Result<PathBuf> {
        if let Some(url) = url {
            let output = Command::new("git")
                .args(["clone", "--depth", "1", url, path.to_str().unwrap()])
                .output()
                .context("Failed to clone repo")?;
            if !output.status.success() {
                eprintln!("Git clone failed: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Ok(path.to_path_buf())
    }
}
