use std::collections::HashMap;
use crate::db::Database;
use crate::claim::VerificationStatus;
use chrono::{DateTime, Utc, TimeDelta};

pub struct AnalyticsEngine {
    db: Database,
}

impl AnalyticsEngine {
    pub fn new(db: Database) -> Self {
        AnalyticsEngine { db }
    }

    pub fn candidate_scorecard(&self, candidate_id: &str) -> Scorecard {
        let stats = self.db.get_candidate_stats(candidate_id).unwrap_or((0, 0, 0));
        let claims = self.db.get_verified_claims(candidate_id).unwrap_or_default();
        let total = stats.0.max(1);
        let pass_rate = stats.1 as f64 / total as f64 * 100.0;

        let mut skill_counts: HashMap<String, u64> = HashMap::new();
        for claim in &claims {
            let word = claim.claim_text.to_lowercase();
            for skill in ["rust", "typescript", "python", "go", "java", "react", "node"] {
                if word.contains(skill) {
                    *skill_counts.entry(skill.to_string()).or_insert(0) += 1;
                }
            }
        }

        let top_skill = skill_counts.iter().max_by_key(|(_, v)| **v).map(|(k, _)| k.clone());

        Scorecard {
            candidate_id: candidate_id.to_string(),
            total_claims: stats.0,
            pass_count: stats.1,
            fail_count: stats.2,
            pass_rate,
            top_skill,
            generated_at: Utc::now(),
        }
    }

    pub fn aggregate_pass_rate(&self) -> AggregateStats {
        let candidates: Vec<(i64, i64, i64)> = vec![];
        AggregateStats {
            total_candidates: 0,
            overall_pass_rate: 0.0,
            avg_claims_per_candidate: 0.0,
            total_claims: 0,
            total_passes: 0,
            total_fails: 0,
        }
    }

    pub fn trend_analysis(&self, _days: i64) -> Vec<TrendPoint> {
        vec![]
    }

    pub fn top_skills(&self) -> Vec<SkillCount> {
        vec![]
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Scorecard {
    pub candidate_id: String,
    pub total_claims: i64,
    pub pass_count: i64,
    pub fail_count: i64,
    pub pass_rate: f64,
    pub top_skill: Option<String>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AggregateStats {
    pub total_candidates: i64,
    pub overall_pass_rate: f64,
    pub avg_claims_per_candidate: f64,
    pub total_claims: i64,
    pub total_passes: i64,
    pub total_fails: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TrendPoint {
    pub date: String,
    pub claims: u64,
    pub passes: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SkillCount {
    pub skill: String,
    pub count: u64,
}