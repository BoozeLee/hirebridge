use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub id: String,
    pub candidate_id: String,
    pub claim_text: String,
    pub repo_url: Option<String>,
    pub code_snippet: Option<String>,
    pub verification_commands: Vec<VerificationCommand>,
    pub status: VerificationStatus,
    pub evidence: Vec<Evidence>,
    pub checksum: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VerificationCommand {
    pub command: String,
    pub expected_output: Option<String>,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum VerificationStatus {
    #[default]
    Pending,
    Pass,
    Fail,
    Unverifiable,
}

impl ToString for VerificationStatus {
    fn to_string(&self) -> String {
        match self {
            VerificationStatus::Pending => "Pending".to_string(),
            VerificationStatus::Pass => "Pass".to_string(),
            VerificationStatus::Fail => "Fail".to_string(),
            VerificationStatus::Unverifiable => "Unverifiable".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Evidence {
    pub command: String,
    pub output: String,
    pub exit_code: i32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub claim_id: String,
    pub status: VerificationStatus,
    pub evidence: Vec<Evidence>,
    pub checksum: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TamperResult {
    pub total: usize,
    pub caught: usize,
    pub missed: usize,
    pub pass_rate: f64,
}

impl Claim {
    pub fn new(candidate_id: &str, claim_text: &str, repo_url: Option<&str>) -> Self {
        let now = Utc::now();
        let mut claim = Claim {
            id: format!("claim-{}", now.timestamp()),
            candidate_id: candidate_id.to_string(),
            claim_text: claim_text.to_string(),
            repo_url: repo_url.map(|s| s.to_string()),
            code_snippet: None,
            verification_commands: Vec::new(),
            status: VerificationStatus::Pending,
            evidence: Vec::new(),
            checksum: String::new(),
            created_at: now,
        };
        claim.checksum = claim.compute_checksum();
        claim
    }

    pub fn compute_checksum(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.claim_text);
        hasher.update(&self.repo_url.clone().unwrap_or_default());
        hasher.update(&self.candidate_id);
        hex::encode(hasher.finalize())
    }

    pub fn add_command(&mut self, command: &str, expected_output: Option<&str>, timeout: u64) {
        self.verification_commands.push(VerificationCommand {
            command: command.to_string(),
            expected_output: expected_output.map(|s| s.to_string()),
            timeout_seconds: timeout,
        });
    }
}
