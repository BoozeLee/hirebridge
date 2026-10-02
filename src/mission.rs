use anyhow::{Result, Context};
use std::collections::HashMap;
use crate::claim::{VerificationStatus, Evidence};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Mission {
    pub id: String,
    pub title: String,
    pub description: String,
    pub mission_type: MissionType,
    pub difficulty: Difficulty,
    pub duration_minutes: u64,
    pub success_criteria: Vec<String>,
    pub failure_criteria: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MissionType {
    StrategicPlanning,
    CrisisManagement,
    Innovation,
    CrossFunctional,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

#[derive(Debug, Clone)]
pub struct MissionResult {
    pub mission_id: String,
    pub candidate_id: String,
    pub status: VerificationStatus,
    pub evidence: Vec<Evidence>,
    pub success_criteria_met: Vec<String>,
    pub failure_criteria_met: Vec<String>,
    pub duration_seconds: u64,
    pub adaptive_challenge_triggered: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct MissionEvaluator {
    missions: HashMap<String, Mission>,
}

impl MissionEvaluator {
    pub fn new() -> Self {
        MissionEvaluator {
            missions: HashMap::new(),
        }
    }

    pub fn add_mission(&mut self, mission: Mission) {
        self.missions.insert(mission.id.clone(), mission);
    }

    pub fn evaluate(&self, mission_id: &str, candidate_id: &str, candidate_output: &str) -> Result<MissionResult> {
        let mission = self.missions.get(mission_id).context("Mission not found")?;

        let mut success_criteria_met = Vec::new();
        let mut failure_criteria_met = Vec::new();

        for criterion in &mission.success_criteria {
            if candidate_output.contains(criterion) {
                success_criteria_met.push(criterion.clone());
            }
        }

        for criterion in &mission.failure_criteria {
            if candidate_output.contains(criterion) {
                failure_criteria_met.push(criterion.clone());
            }
        }

        let passed = !success_criteria_met.is_empty()
            && failure_criteria_met.is_empty();

        let adaptive_challenge = self.check_adaptive_challenge(mission, candidate_output);

        Ok(MissionResult {
            mission_id: mission_id.to_string(),
            candidate_id: candidate_id.to_string(),
            status: if passed { VerificationStatus::Pass } else { VerificationStatus::Fail },
            evidence: vec![Evidence {
                command: format!("evaluate-mission --mission {}", mission_id),
                output: format!(
                    "Success criteria met: {}\nFailure criteria met: {}\nAdaptive challenge: {}",
                    success_criteria_met.len(),
                    failure_criteria_met.len(),
                    adaptive_challenge
                ),
                exit_code: if passed { 0 } else { 1 },
                timestamp: Utc::now(),
            }],
            success_criteria_met,
            failure_criteria_met,
            duration_seconds: 0,
            adaptive_challenge_triggered: adaptive_challenge,
            timestamp: Utc::now(),
        })
    }

    fn check_adaptive_challenge(&self, mission: &Mission, candidate_output: &str) -> bool {
        // Trigger adaptive challenge if candidate is performing well
        mission.success_criteria.iter().any(|c| candidate_output.contains(c))
            && candidate_output.len() > 100
    }

    pub fn create_mission(
        id: &str,
        title: &str,
        mission_type: MissionType,
        difficulty: Difficulty,
    ) -> Mission {
        Mission {
            id: id.to_string(),
            title: title.to_string(),
            description: format!("{:?} mission at {:?} difficulty", mission_type, difficulty),
            mission_type,
            difficulty,
            duration_minutes: 60,
            success_criteria: vec![],
            failure_criteria: vec![],
        }
    }
}
