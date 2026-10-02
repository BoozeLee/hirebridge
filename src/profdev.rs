use crate::claim::Evidence;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct CompetencyProfile {
    pub candidate_id: String,
    pub skills: Vec<SkillProficiency>,
    pub experience_years: f64,
    pub certifications: Vec<String>,
    pub role_target: String,
}

#[derive(Debug, Clone)]
pub struct SkillProficiency {
    pub name: String,
    pub level: SkillLevel,
    pub years_experience: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone)]
pub struct CompetencyGap {
    pub skill_name: String,
    pub required_level: SkillLevel,
    pub current_level: SkillLevel,
    pub gap_magnitude: u8,
}

#[derive(Debug, Clone)]
pub struct MicroLearningResource {
    pub id: String,
    pub title: String,
    pub skill_name: String,
    pub resource_type: ResourceType,
    pub duration_minutes: u64,
    pub url: Option<String>,
    pub difficulty: SkillLevel,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    Video,
    Article,
    Interactive,
    Quiz,
    Project,
}

#[derive(Debug, Clone)]
pub struct LearningPlan {
    pub candidate_id: String,
    pub gaps: Vec<CompetencyGap>,
    pub resources: Vec<MicroLearningResource>,
    pub total_duration_minutes: u64,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct Mentor {
    pub id: String,
    pub name: String,
    pub skills: Vec<SkillProficiency>,
    pub years_experience: u8,
    pub availability: Vec<String>,
    pub mentorship_style: String,
}

#[derive(Debug, Clone)]
pub struct MentorshipMatch {
    pub candidate_id: String,
    pub mentor_id: String,
    pub mentor_name: String,
    pub match_score: f64,
    pub shared_skills: Vec<String>,
    pub skill_gap_coverage: Vec<CompetencyGap>,
    pub recommended_focus: String,
}

pub struct ProfessionalDevEngine {
    learning_resources: Vec<MicroLearningResource>,
    mentors: Vec<Mentor>,
}

impl ProfessionalDevEngine {
    pub fn new() -> Self {
        let resources = vec![
            MicroLearningResource {
                id: "ml-001".to_string(),
                title: "Rust Ownership Fundamentals".to_string(),
                skill_name: "Rust".to_string(),
                resource_type: ResourceType::Video,
                duration_minutes: 15,
                url: Some("https://example.com/rust-ownership".to_string()),
                difficulty: SkillLevel::Beginner,
            },
            MicroLearningResource {
                id: "ml-002".to_string(),
                title: "Advanced Tokio Async Patterns".to_string(),
                skill_name: "Rust".to_string(),
                resource_type: ResourceType::Article,
                duration_minutes: 20,
                url: Some("https://example.com/tokio-async".to_string()),
                difficulty: SkillLevel::Expert,
            },
            MicroLearningResource {
                id: "ml-003".to_string(),
                title: "TypeScript Generics Deep Dive".to_string(),
                skill_name: "TypeScript".to_string(),
                resource_type: ResourceType::Interactive,
                duration_minutes: 25,
                url: Some("https://example.com/ts-generics".to_string()),
                difficulty: SkillLevel::Intermediate,
            },
            MicroLearningResource {
                id: "ml-004".to_string(),
                title: "System Design Basics".to_string(),
                skill_name: "Architecture".to_string(),
                resource_type: ResourceType::Video,
                duration_minutes: 30,
                url: Some("https://example.com/system-design".to_string()),
                difficulty: SkillLevel::Intermediate,
            },
            MicroLearningResource {
                id: "ml-005".to_string(),
                title: "Python Performance Tuning".to_string(),
                skill_name: "Python".to_string(),
                resource_type: ResourceType::Project,
                duration_minutes: 45,
                url: Some("https://example.com/python-perf".to_string()),
                difficulty: SkillLevel::Advanced,
            },
            MicroLearningResource {
                id: "ml-006".to_string(),
                title: "CI/CD Pipeline Patterns".to_string(),
                skill_name: "DevOps".to_string(),
                resource_type: ResourceType::Interactive,
                duration_minutes: 20,
                url: Some("https://example.com/cicd-patterns".to_string()),
                difficulty: SkillLevel::Intermediate,
            },
            MicroLearningResource {
                id: "ml-007".to_string(),
                title: "GraphQL Schema Design".to_string(),
                skill_name: "GraphQL".to_string(),
                resource_type: ResourceType::Quiz,
                duration_minutes: 10,
                url: Some("https://example.com/graphql-schema".to_string()),
                difficulty: SkillLevel::Beginner,
            },
            MicroLearningResource {
                id: "ml-008".to_string(),
                title: "PostgreSQL Optimization".to_string(),
                skill_name: "Database".to_string(),
                resource_type: ResourceType::Article,
                duration_minutes: 25,
                url: Some("https://example.com/pg-optimization".to_string()),
                difficulty: SkillLevel::Advanced,
            },
        ];

        let mentors = vec![
            Mentor {
                id: "mentor-001".to_string(),
                name: "Alice Chen".to_string(),
                skills: vec![
                    SkillProficiency { name: "Rust".to_string(), level: SkillLevel::Expert, years_experience: 6.0 },
                    SkillProficiency { name: "TypeScript".to_string(), level: SkillLevel::Advanced, years_experience: 4.0 },
                ],
                years_experience: 8,
                availability: vec!["Monday".to_string(), "Wednesday".to_string()],
                mentorship_style: "Structured".to_string(),
            },
            Mentor {
                id: "mentor-002".to_string(),
                name: "Bob Martinez".to_string(),
                skills: vec![
                    SkillProficiency { name: "Python".to_string(), level: SkillLevel::Expert, years_experience: 7.0 },
                    SkillProficiency { name: "DevOps".to_string(), level: SkillLevel::Advanced, years_experience: 5.0 },
                ],
                years_experience: 10,
                availability: vec!["Tuesday".to_string(), "Thursday".to_string()],
                mentorship_style: "Hands-on".to_string(),
            },
            Mentor {
                id: "mentor-003".to_string(),
                name: "Carol Singh".to_string(),
                skills: vec![
                    SkillProficiency { name: "Architecture".to_string(), level: SkillLevel::Expert, years_experience: 9.0 },
                    SkillProficiency { name: "GraphQL".to_string(), level: SkillLevel::Advanced, years_experience: 3.0 },
                    SkillProficiency { name: "Database".to_string(), level: SkillLevel::Advanced, years_experience: 5.0 },
                ],
                years_experience: 12,
                availability: vec!["Friday".to_string(), "Monday".to_string()],
                mentorship_style: "Big-picture".to_string(),
            },
        ];

        ProfessionalDevEngine {
            learning_resources: resources,
            mentors,
        }
    }

    pub fn analyze_gaps(
        &self,
        candidate: &CompetencyProfile,
        required_skills: &[SkillProficiency],
    ) -> Vec<CompetencyGap> {
        let mut gaps = Vec::new();

        for required in required_skills {
            let current = candidate.skills.iter().find(|s| s.name == required.name);
            match current {
                Some(current_skill) => {
                    let gap = self.level_difference(&current_skill.level, &required.level);
                    if gap > 0 {
                        gaps.push(CompetencyGap {
                            skill_name: required.name.clone(),
                            required_level: required.level.clone(),
                            current_level: current_skill.level.clone(),
                            gap_magnitude: gap,
                        });
                    }
                }
                None => {
                    gaps.push(CompetencyGap {
                        skill_name: required.name.clone(),
                        required_level: required.level.clone(),
                        current_level: SkillLevel::Beginner,
                        gap_magnitude: 3,
                    });
                }
            }
        }

        gaps
    }

    fn level_difference(&self, current: &SkillLevel, required: &SkillLevel) -> u8 {
        let current_val = self.level_value(current);
        let required_val = self.level_value(required);
        if required_val > current_val {
            required_val - current_val
        } else {
            0
        }
    }

    fn level_value(&self, level: &SkillLevel) -> u8 {
        match level {
            SkillLevel::Beginner => 1,
            SkillLevel::Intermediate => 2,
            SkillLevel::Advanced => 3,
            SkillLevel::Expert => 4,
        }
    }

    pub fn recommend_learning(
        &self,
        gaps: &[CompetencyGap],
    ) -> Vec<MicroLearningResource> {
        let mut resources = Vec::new();
        for gap in gaps {
            let matching: Vec<&MicroLearningResource> = self.learning_resources
                .iter()
                .filter(|r| r.skill_name == gap.skill_name)
                .filter(|r| self.level_value(&r.difficulty) <= self.level_value(&gap.required_level))
                .collect();
            resources.extend(matching.into_iter().cloned());
        }
        resources
    }

    pub fn generate_learning_plan(
        &self,
        candidate_id: &str,
        gaps: Vec<CompetencyGap>,
    ) -> LearningPlan {
        let resources = self.recommend_learning(&gaps);
        let total_duration: u64 = resources.iter().map(|r| r.duration_minutes).sum();

        LearningPlan {
            candidate_id: candidate_id.to_string(),
            gaps,
            resources,
            total_duration_minutes: total_duration,
            generated_at: Utc::now(),
        }
    }

    pub fn find_mentors(
        &self,
        gaps: &[CompetencyGap],
    ) -> Vec<MentorshipMatch> {
        let gap_skills: Vec<String> = gaps.iter().map(|g| g.skill_name.clone()).collect();
        let mut matches = Vec::new();

        for mentor in &self.mentors {
            let shared: Vec<String> = mentor.skills.iter()
                .filter(|s| gap_skills.contains(&s.name))
                .map(|s| s.name.clone())
                .collect();

            if shared.is_empty() {
                continue;
            }

            let skill_score = shared.len() as f64 / gap_skills.len().max(1) as f64;
            let experience_bonus = (mentor.years_experience as f64 / 20.0).min(0.3);
            let match_score = (skill_score * 0.7 + experience_bonus).min(1.0);

            let covered_gaps: Vec<CompetencyGap> = gaps.iter()
                .filter(|g| shared.contains(&g.skill_name))
                .cloned()
                .collect();

            let focus = if mentor.years_experience >= 10 {
                "Leadership and architecture".to_string()
            } else if mentor.years_experience >= 5 {
                "Technical depth and best practices".to_string()
            } else {
                "Core skill development".to_string()
            };

            matches.push(MentorshipMatch {
                candidate_id: String::new(),
                mentor_id: mentor.id.clone(),
                mentor_name: mentor.name.clone(),
                match_score,
                shared_skills: shared,
                skill_gap_coverage: covered_gaps,
                recommended_focus: focus,
            });
        }

        matches.sort_by(|a, b| b.match_score.partial_cmp(&a.match_score).unwrap());
        matches
    }

    pub fn assess_candidate(
        &self,
        candidate: &CompetencyProfile,
        required_skills: &[SkillProficiency],
    ) -> Evidence {
        let gaps = self.analyze_gaps(candidate, required_skills);
        let learning_plan = self.generate_learning_plan(&candidate.candidate_id, gaps);

        Evidence {
            command: format!(
                "assess-candidate --candidate {} --role {}",
                candidate.candidate_id, candidate.role_target
            ),
            output: format!(
                "Gaps found: {}\nTotal learning duration: {} min\nResources recommended: {}",
                learning_plan.gaps.len(),
                learning_plan.total_duration_minutes,
                learning_plan.resources.len()
            ),
            exit_code: if learning_plan.gaps.is_empty() { 0 } else { 1 },
            timestamp: Utc::now(),
        }
    }
}