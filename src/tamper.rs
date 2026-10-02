use anyhow::{Result, Context};
use std::fs;
use std::path::PathBuf;
use crate::claim::{Claim, VerificationStatus, TamperResult};
use crate::verify::Verifier;

pub enum Mutation {
    RemoveLine { path: PathBuf, line_number: usize },
    ChangeString { path: PathBuf, from: String, to: String },
    DeleteFile { path: PathBuf },
    AddFakeOutput { path: PathBuf, content: String },
    TamperChecksum { path: PathBuf },
    SilenceFailure { path: PathBuf },
}

impl Mutation {
    pub fn apply(&self) {
        match self {
            Mutation::RemoveLine { path, line_number } => {
                if let Ok(content) = fs::read_to_string(path) {
                    let lines: Vec<&str> = content.lines().collect();
                    if *line_number < lines.len() {
                        let mut new_lines = lines.clone();
                        new_lines.remove(*line_number);
                        fs::write(path, new_lines.join("\n")).ok();
                    }
                }
            }
            Mutation::ChangeString { path, from, to } => {
                if let Ok(content) = fs::read_to_string(path) {
                    let new_content = content.replace(from, to);
                    fs::write(path, new_content).ok();
                }
            }
            Mutation::DeleteFile { path } => {
                fs::remove_file(path).ok();
            }
            Mutation::AddFakeOutput { path, content } => {
                fs::write(path, content).ok();
            }
            Mutation::TamperChecksum { path } => {
                if let Ok(content) = fs::read_to_string(path) {
                    let tampered = format!("{}TAMPERED", content);
                    fs::write(path, tampered).ok();
                }
            }
            Mutation::SilenceFailure { path } => {
                if let Ok(content) = fs::read_to_string(path) {
                    let new_content = content.replace("exit 1", "exit 0");
                    fs::write(path, new_content).ok();
                }
            }
        }
    }

    pub fn revert(&self) {
        // Revert is a no-op for this demo; in production, use git checkout
    }
}

pub struct TamperDetector {
    mutations: Vec<Mutation>,
}

impl TamperDetector {
    pub fn new() -> Self {
        TamperDetector {
            mutations: Vec::new(),
        }
    }

    pub fn seed_mutations(&self, repo_path: &PathBuf) -> Vec<Mutation> {
        vec![
            Mutation::RemoveLine { path: repo_path.join("src/main.rs"), line_number: 42 },
            Mutation::ChangeString { path: repo_path.join("src/main.rs"), from: "PASS".to_string(), to: "FAIL".to_string() },
            Mutation::DeleteFile { path: repo_path.join("Cargo.toml") },
            Mutation::AddFakeOutput { path: repo_path.join("tests/output.txt"), content: "FAKE OUTPUT".to_string() },
            Mutation::TamperChecksum { path: repo_path.join("checksums.json") },
            Mutation::SilenceFailure { path: repo_path.join("src/lib.rs") },
        ]
    }

    pub fn run_tamper_suite(&self, claim: &Claim) -> Result<TamperResult> {
        let verifier = Verifier::new("./claims", "./sandbox");
        let temp_dir = tempfile::tempdir().context("Failed to create temp dir")?;
        let repo_path = if let Some(url) = &claim.repo_url {
            let output = std::process::Command::new("git")
                .args(["clone", "--depth", "1", url, temp_dir.path().to_str().unwrap()])
                .output()
                .context("Failed to clone repo")?;
            if !output.status.success() {
                temp_dir.path().to_path_buf()
            } else {
                temp_dir.path().to_path_buf()
            }
        } else {
            temp_dir.path().to_path_buf()
        };

        let mutations = self.seed_mutations(&repo_path);
        let total = mutations.len();
        let mut caught = 0;

        for mutation in &mutations {
            mutation.apply();
            let result = verifier.verify(claim)?;
            if result.status == VerificationStatus::Fail {
                caught += 1;
            }
            mutation.revert();
        }

        Ok(TamperResult {
            total,
            caught,
            missed: total - caught,
            pass_rate: caught as f64 / total as f64,
        })
    }
}


