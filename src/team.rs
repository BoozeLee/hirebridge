use std::collections::HashMap;
use crate::claim::{VerificationStatus, Evidence};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct TeamMember {
    pub candidate_id: String,
    pub name: String,
    pub role: String,
    pub skills: Vec<String>,
    pub communication_style: CommunicationStyle,
    pub leadership_score: u8,
    pub collaboration_score: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommunicationStyle {
    Direct,
    Analytical,
    Creative,
    Structured,
    Adaptive,
}

#[derive(Debug, Clone)]
pub struct TeamFormation {
    pub id: String,
    pub members: Vec<TeamMember>,
    pub compatibility_score: f64,
    pub role_assignment: HashMap<String, String>,
    pub collaboration_evidence: Vec<Evidence>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct TeamChallenge {
    pub id: String,
    pub title: String,
    pub description: String,
    pub required_skills: Vec<String>,
    pub min_members: usize,
    pub max_members: usize,
    pub duration_minutes: u64,
}

pub struct TeamBuilder {
    members: HashMap<String, TeamMember>,
    challenges: Vec<TeamChallenge>,
}

impl TeamBuilder {
    pub fn new() -> Self {
        TeamBuilder {
            members: HashMap::new(),
            challenges: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: TeamMember) {
        self.members.insert(member.candidate_id.clone(), member);
    }

    pub fn add_challenge(&mut self, challenge: TeamChallenge) {
        self.challenges.push(challenge);
    }

    pub fn form_team(&self, challenge_id: &str) -> Option<TeamFormation> {
        let challenge = self.challenges.iter().find(|c| c.id == challenge_id)?;
        let mut candidates: Vec<&TeamMember> = self.members.values().collect();

        // Filter by required skills
        candidates.retain(|m| {
            challenge.required_skills.iter().all(|skill| m.skills.contains(skill))
        });

        if candidates.len() < challenge.min_members {
            return None;
        }

        // Sort by collaboration score (highest first)
        candidates.sort_by(|a, b| b.collaboration_score.cmp(&a.collaboration_score));

        // Select top candidates up to max_members
        let selected: Vec<TeamMember> = candidates
            .into_iter()
            .take(challenge.max_members)
            .cloned()
            .collect();

        // Calculate compatibility score
        let compatibility_score = self.calculate_compatibility(&selected);

        // Assign roles based on skills and leadership score
        let mut role_assignment = HashMap::new();
        for member in &selected {
            let role = self.assign_role(member, &challenge);
            role_assignment.insert(member.candidate_id.clone(), role);
        }

        Some(TeamFormation {
            id: format!("team-{}", Utc::now().timestamp()),
            members: selected,
            compatibility_score,
            role_assignment,
            collaboration_evidence: Vec::new(),
            timestamp: Utc::now(),
        })
    }

    fn calculate_compatibility(&self, members: &[TeamMember]) -> f64 {
        if members.is_empty() {
            return 0.0;
        }

        let avg_collaboration = members.iter()
            .map(|m| m.collaboration_score as f64)
            .sum::<f64>() / members.len() as f64;

        let avg_leadership = members.iter()
            .map(|m| m.leadership_score as f64)
            .sum::<f64>() / members.len() as f64;

        // Diversity bonus: different communication styles increase compatibility
        let styles: Vec<&CommunicationStyle> = members.iter()
            .map(|m| &m.communication_style)
            .collect();
        let unique_styles = styles.iter().collect::<std::collections::HashSet<_>>().len();
        let diversity_bonus = (unique_styles as f64 / styles.len() as f64) * 0.2;

        (avg_collaboration * 0.5 + avg_leadership * 0.3 + 50.0 * 0.2) / 100.0 + diversity_bonus
    }

    fn assign_role(&self, member: &TeamMember, challenge: &TeamChallenge) -> String {
        if member.leadership_score >= 8 && challenge.min_members > 2 {
            "Leader".to_string()
        } else if member.skills.contains(&"Rust".to_string()) {
            "Technical Lead".to_string()
        } else if member.skills.contains(&"TypeScript".to_string()) {
            "Frontend Lead".to_string()
        } else if member.skills.contains(&"Python".to_string()) {
            "Backend Lead".to_string()
        } else {
            "Contributor".to_string()
        }
    }

    pub fn assess_collaboration(&self, team: &TeamFormation) -> Vec<Evidence> {
        let mut evidence = Vec::new();

        for member in &team.members {
            let collaboration_evidence = Evidence {
                command: format!("assess-collaboration --candidate {}", member.candidate_id),
                output: format!(
                    "Collaboration score: {}, Leadership score: {}, Style: {:?}",
                    member.collaboration_score, member.leadership_score, member.communication_style
                ),
                exit_code: 0,
                timestamp: Utc::now(),
            };
            evidence.push(collaboration_evidence);
        }

        evidence
    }
}
