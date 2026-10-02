use hirebridge::claim::{Claim, VerificationStatus, Evidence};
use hirebridge::db::Database;
use hirebridge::tamper::TamperDetector;
use hirebridge::mission::{MissionEvaluator, Mission, MissionType, Difficulty};
use hirebridge::profdev::{ProfessionalDevEngine, CompetencyProfile, SkillProficiency, SkillLevel, CompetencyGap};
use hirebridge::github::GitHubClient;
use std::path::PathBuf;

#[test]
fn claim_compute_checksum_is_deterministic() {
    let c1 = Claim::new("cand-1", "I know Rust", Some("https://github.com/test/repo"));
    let c2 = Claim::new("cand-1", "I know Rust", Some("https://github.com/test/repo"));
    assert_eq!(c1.checksum, c2.checksum);
}

#[test]
fn claim_checksum_changes_with_text() {
    let c1 = Claim::new("cand-1", "I know Rust", None);
    let c2 = Claim::new("cand-1", "I know Python", None);
    assert_ne!(c1.checksum, c2.checksum);
}

#[test]
fn verification_status_default_is_pending() {
    assert_eq!(VerificationStatus::default(), VerificationStatus::Pending);
}

#[test]
fn tamper_detector_seeds_six_mutations() {
    let detector = TamperDetector::new();
    let repo_path = PathBuf::from("/tmp/test-repo");
    let mutations = detector.seed_mutations(&repo_path);
    assert_eq!(mutations.len(), 6);
}

#[test]
fn mission_evaluator_add_and_evaluate() {
    let mut evaluator = MissionEvaluator::new();
    let mission = Mission {
        id: "test-mission".to_string(),
        title: "Test Mission".to_string(),
        description: "A test".to_string(),
        mission_type: MissionType::StrategicPlanning,
        difficulty: Difficulty::Medium,
        duration_minutes: 30,
        success_criteria: vec!["completed".to_string()],
        failure_criteria: vec!["failed".to_string()],
    };
    evaluator.add_mission(mission);
    let result = evaluator.evaluate("test-mission", "cand-1", "completed successfully with a longer output that exceeds 100 characters threshold for triggering adaptive challenge scenarios in the mission evaluation system").unwrap();
    assert_eq!(result.status, VerificationStatus::Pass);
    assert!(result.adaptive_challenge_triggered);
}

#[test]
fn mission_evaluator_fails_on_failure_criterion() {
    let mut evaluator = MissionEvaluator::new();
    let mission = Mission {
        id: "test-mission-2".to_string(),
        title: "Test Mission 2".to_string(),
        description: "A test".to_string(),
        mission_type: MissionType::StrategicPlanning,
        difficulty: Difficulty::Medium,
        duration_minutes: 30,
        success_criteria: vec!["completed".to_string()],
        failure_criteria: vec!["failed".to_string()],
    };
    evaluator.add_mission(mission);
    let result = evaluator.evaluate("test-mission-2", "cand-1", "completed but failed").unwrap();
    assert_eq!(result.status, VerificationStatus::Fail);
}

#[test]
fn profdev_analyzes_gaps_correctly() {
    let engine = ProfessionalDevEngine::new();
    let candidate = CompetencyProfile {
        candidate_id: "cand-1".to_string(),
        skills: vec![
            SkillProficiency { name: "Rust".to_string(), level: SkillLevel::Advanced, years_experience: 3.0 },
        ],
        experience_years: 4.0,
        certifications: vec![],
        role_target: "Engineer".to_string(),
    };
    let required = vec![
        SkillProficiency { name: "Rust".to_string(), level: SkillLevel::Expert, years_experience: 5.0 },
        SkillProficiency { name: "Python".to_string(), level: SkillLevel::Intermediate, years_experience: 2.0 },
    ];
    let gaps = engine.analyze_gaps(&candidate, &required);
    assert_eq!(gaps.len(), 2);
    assert!(gaps.iter().any(|g| g.skill_name == "Rust" && g.gap_magnitude == 1));
    assert!(gaps.iter().any(|g| g.skill_name == "Python" && g.gap_magnitude == 3));
}

#[test]
fn profdev_recommends_learning_for_gaps() {
    let engine = ProfessionalDevEngine::new();
    let gaps = vec![
        CompetencyGap {
            skill_name: "Rust".to_string(),
            required_level: SkillLevel::Intermediate,
            current_level: SkillLevel::Beginner,
            gap_magnitude: 2,
        },
    ];
    let resources = engine.recommend_learning(&gaps);
    assert!(resources.len() > 0);
    assert!(resources.iter().all(|r| r.skill_name == "Rust"));
}

#[test]
fn profdev_finds_mentors_for_gap_skills() {
    let engine = ProfessionalDevEngine::new();
    let gaps = vec![
        CompetencyGap {
            skill_name: "Rust".to_string(),
            required_level: SkillLevel::Advanced,
            current_level: SkillLevel::Beginner,
            gap_magnitude: 2,
        },
    ];
    let matches = engine.find_mentors(&gaps);
    assert!(matches.len() > 0);
    assert!(matches[0].match_score > 0.0);
}

#[test]
fn github_client_repo_path_parses_url() {
    let client = GitHubClient::new().unwrap();
    assert_eq!(
        client.repo_path("https://github.com/user/repo.git"),
        "user/repo"
    );
    assert_eq!(
        client.repo_path("git@github.com:org/project.git"),
        "org/project"
    );
    assert_eq!(
        client.repo_path("https://github.com/foo/bar"),
        "foo/bar"
    );
}