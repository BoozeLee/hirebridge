use anyhow::Result;
use serde::Deserialize;
use chrono::Utc;
use crate::claim::{Claim, Evidence};

#[derive(Debug, Deserialize)]
pub struct GitHubCommit {
    pub sha: String,
    pub commit: GitHubCommitInfo,
    pub author: Option<GitHubAuthor>,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubCommitInfo {
    pub message: String,
    pub author: GitHubCommitAuthor,
}

#[derive(Debug, Deserialize)]
pub struct GitHubCommitAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubAuthor {
    pub login: String,
    pub id: u64,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubPR {
    pub number: u64,
    pub title: String,
    pub state: String,
    pub merged: bool,
    pub created_at: String,
    pub merged_at: Option<String>,
    pub user: GitHubAuthor,
}

#[derive(Debug, Deserialize)]
pub struct GitHubContribution {
    pub total: u64,
    pub weeks: Vec<GitHubContributionWeek>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubContributionWeek {
    pub w: i64,
    pub a: u64,
    pub d: u64,
    pub c: u64,
}

pub struct GitHubClient {
    client: reqwest::Client,
    base_url: String,
    token: Option<String>,
}

impl GitHubClient {
    pub fn new() -> Result<Self> {
        let token = std::env::var("GITHUB_TOKEN").ok();
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(ref t) = token {
            let value = reqwest::header::HeaderValue::from_str(
                &format!("token {}", t)
            ).map_err(|e| anyhow::anyhow!("Invalid token header: {}", e))?;
            headers.insert(reqwest::header::AUTHORIZATION, value);
        }
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create HTTP client: {}", e))?;
        Ok(GitHubClient {
            client,
            base_url: "https://api.github.com".to_string(),
            token,
        })
    }

    pub async fn fetch_commits(
        &self,
        repo_url: &str,
        candidate_email: &str,
    ) -> Result<Vec<GitHubCommit>> {
        let path = self.repo_path(repo_url);
        let url = format!("{}/repos/{}/commits", self.base_url, path);
        let response = self.client
            .get(&url)
            .header("User-Agent", "hirebridge")
            .query(&[("per_page", "100")])
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch commits: {}", e))?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        let commits: Vec<GitHubCommit> = response.json().await
            .map_err(|e| anyhow::anyhow!("Failed to parse commits: {}", e))?;
        let filtered: Vec<GitHubCommit> = commits.into_iter()
            .filter(|c| c.author.as_ref().map_or(false, |a| a.login == candidate_email))
            .collect();
        Ok(filtered)
    }

    pub async fn fetch_pr_history(
        &self,
        repo_url: &str,
        candidate_login: &str,
    ) -> Result<Vec<GitHubPR>> {
        let path = self.repo_path(repo_url);
        let url = format!("{}/repos/{}/pulls", self.base_url, path);
        let response = self.client
            .get(&url)
            .header("User-Agent", "hirebridge")
            .query(&[("state", "all"), ("per_page", "100")])
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch PRs: {}", e))?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        let prs: Vec<GitHubPR> = response.json().await
            .map_err(|e| anyhow::anyhow!("Failed to parse PRs: {}", e))?;
        let filtered: Vec<GitHubPR> = prs.into_iter()
            .filter(|pr| pr.user.login == candidate_login)
            .collect();
        Ok(filtered)
    }

    pub async fn fetch_contributions(
        &self,
        candidate_login: &str,
    ) -> Result<GitHubContribution> {
        let url = format!("{}/users/{}/repos", self.base_url, candidate_login);
        let response = self.client
            .get(&url)
            .header("User-Agent", "hirebridge")
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch contributions: {}", e))?;

        if !response.status().is_success() {
            return Ok(GitHubContribution { total: 0, weeks: Vec::new() });
        }

        let repos: Vec<GitHubRepo> = response.json().await
            .map_err(|e| anyhow::anyhow!("Failed to parse repos: {}", e))?;
        let total = repos.iter().map(|r| r.stargazers_count + r.forks_count).sum();
        Ok(GitHubContribution { total, weeks: Vec::new() })
    }

    pub fn verify_repo_activity(&self, claim: &Claim, commits: &[GitHubCommit]) -> Evidence {
        let candidate_id = &claim.candidate_id;
        let commit_count = commits.len();
        let has_commits = commit_count > 0;

        Evidence {
            command: format!("github-sync --candidate {} --repo {:?}", candidate_id, claim.repo_url),
            output: format!(
                "Commits found: {}\nCandidate: {}\nRepo: {:?}",
                commit_count, candidate_id, claim.repo_url
            ),
            exit_code: if has_commits { 0 } else { 1 },
            timestamp: Utc::now(),
        }
    }

    pub fn repo_path(&self, repo_url: &str) -> String {
        repo_url
            .strip_suffix(".git")
            .unwrap_or(repo_url)
            .replace("https://github.com/", "")
            .replace("git@github.com:", "")
    }
}

#[derive(Debug, Deserialize)]
struct GitHubRepo {
    pub stargazers_count: u64,
    pub forks_count: u64,
}