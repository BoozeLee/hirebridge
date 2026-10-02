use anyhow::{Result, Context};
use std::process::Command;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use crate::claim::{Claim, VerificationStatus, VerificationResult, Evidence};
use chrono::Utc;

pub struct Verifier {
    claims_dir: String,
    sandbox_dir: String,
}

impl Verifier {
    pub fn new(claims_dir: &str, sandbox_dir: &str) -> Self {
        Verifier {
            claims_dir: claims_dir.to_string(),
            sandbox_dir: sandbox_dir.to_string(),
        }
    }

    pub fn verify(&self, claim: &Claim) -> Result<VerificationResult> {
        let temp_dir = TempDir::new().context("Failed to create temp dir")?;
        let repo_path = self.clone_repo(&claim.repo_url, temp_dir.path())?;

        let mut evidence = Vec::new();
        let mut all_passed = true;

        for cmd in &claim.verification_commands {
            let output = Command::new("sh")
                .arg("-c")
                .arg(&cmd.command)
                .current_dir(&repo_path)
                .output()
                .context("Failed to execute command")?;

            let exit_code = output.status.code().unwrap_or(-1);
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            let passed = exit_code == 0
                && cmd.expected_output.as_ref()
                    .map(|expected| stdout.contains(expected))
                    .unwrap_or(true);

            evidence.push(Evidence {
                command: cmd.command.clone(),
                output: format!("{}\n{}", stdout, stderr),
                exit_code,
                timestamp: Utc::now(),
            });

            if !passed {
                all_passed = false;
            }
        }

        Ok(VerificationResult {
            claim_id: claim.id.clone(),
            status: if all_passed { VerificationStatus::Pass } else { VerificationStatus::Fail },
            evidence,
            checksum: claim.checksum.clone(),
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


