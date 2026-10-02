mod claim;
mod verify;
mod tamper;
mod db;
mod harness;
mod team;
mod mission;
mod profdev;

use clap::{Parser, Subcommand};
use hirebridge::claim::{Claim, VerificationStatus};
use hirebridge::db::Database;
use hirebridge::verify::Verifier;
use hirebridge::tamper::TamperDetector;
use hirebridge::harness::{Harness, Task, ExpectedOutcome};
use hirebridge::team::{TeamBuilder, TeamMember, CommunicationStyle, TeamChallenge};
use hirebridge::mission::{MissionEvaluator, Mission, MissionType, Difficulty};
use hirebridge::profdev::{ProfessionalDevEngine, CompetencyProfile, SkillProficiency, SkillLevel, CompetencyGap, ResourceType};
use hirebridge::github::GitHubClient;
use hirebridge::serve::start_server;

#[derive(Parser)]
#[command(name = "hirebridge")]
#[command(about = "AI-powered hiring verification tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Submit a claim for verification
    Claim {
        text: String,
        #[arg(long)]
        repo: Option<String>,
        #[arg(long)]
        candidate: String,
    },
    /// Verify a claim
    Verify {
        claim_id: String,
        #[arg(long)]
        clean_clone: bool,
    },
    /// List verified claims
    Claims {
        #[arg(long)]
        candidate: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },
    /// Generate a report
    Report {
        candidate_id: String,
        #[arg(long, default_value = "json")]
        format: String,
    },
    /// Run tamper detection
    Tamper {
        claim_id: String,
    },
    /// Compare candidates
    Compare {
        candidate_a: String,
        candidate_b: String,
    },
    /// Execute a task in sandbox
    Task {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        description: String,
        #[arg(long)]
        repo: Option<String>,
        #[arg(long, num_args = 1..)]
        setup: Vec<String>,
        #[arg(long, num_args = 1..)]
        verify: Vec<String>,
        #[arg(long, default_value = "60")]
        timeout: u64,
        #[arg(long)]
        candidate: String,
        #[arg(long, default_value = "pass")]
        outcome: String,
        #[arg(long)]
        expected: Option<String>,
        #[arg(long, default_value = "0")]
        exit_code: i32,
    },
    /// Form a team from candidates
    Team {
        #[arg(short, long)]
        challenge: String,
        #[arg(long, num_args = 1..)]
        candidates: Vec<String>,
    },
    /// Assess candidate collaboration
    Assess {
        #[arg(long)]
        candidate: String,
        #[arg(long)]
        collaboration: u8,
        #[arg(long)]
        leadership: u8,
        #[arg(long)]
        style: String,
    },
    /// Evaluate a mission
    Mission {
        #[arg(short, long)]
        title: String,
        #[arg(long, default_value = "strategic")]
        mtype: String,
        #[arg(long, default_value = "medium")]
        difficulty: String,
        #[arg(long)]
        candidate: String,
        #[arg(long)]
        output: String,
    },
    /// Analyze competency gaps
    AssessCandidate {
        #[arg(long)]
        candidate: String,
        #[arg(long, num_args = 1..)]
        skills: Vec<String>,
        #[arg(long)]
        role: String,
    },
    /// Generate a learning plan
    LearningPlan {
        #[arg(long)]
        candidate: String,
        #[arg(long, num_args = 1..)]
        gaps: Vec<String>,
    },
    /// Find mentorship matches
    MentorMatch {
        #[arg(long)]
        candidate: String,
        #[arg(long, num_args = 1..)]
        skills: Vec<String>,
    },
    /// Sync with GitHub
    GitHubSync {
        #[arg(long)]
        candidate: String,
        #[arg(long)]
        repo: String,
        #[arg(long)]
        email: String,
    },
    /// Start web dashboard server
    Serve {
        #[arg(long, default_value = "127.0.0.1:8080")]
        addr: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Claim { text, repo, candidate } => {
            let mut claim = Claim::new(&candidate, &text, repo.as_deref());
            claim.status = VerificationStatus::Pending;
            let db = Database::new("hirebridge.db").expect("Failed to open database");
            db.insert_claim(&claim).expect("Failed to insert claim");
            println!("Claim submitted: {}", claim.id);
        }
        Commands::Verify { claim_id, clean_clone: _ } => {
            let verifier = Verifier::new("./claims", "./sandbox");
            let db = Database::new("hirebridge.db").expect("Failed to open database");
            let claims = db.get_verified_claims(&claim_id).expect("Failed to get claims");
            if claims.is_empty() {
                eprintln!("Claim not found: {}", claim_id);
                return;
            }
            let result = verifier.verify(&claims[0]);
            println!("Verification result: {:?}", result.status);
            for evidence in &result.evidence {
                println!("  {}: {} (exit {})", evidence.command, evidence.output.trim(), evidence.exit_code);
            }
        }
        Commands::Claims { candidate, status: _ } => {
            let db = Database::new("hirebridge.db").expect("Failed to open database");
            if let Some(ref c) = candidate {
                let claims = db.get_verified_claims(c).expect("Failed to get claims");
                for claim in claims {
                    println!("{}: {} — {:?}", claim.id, claim.claim_text, claim.status);
                }
            } else {
                println!("Please provide a candidate ID with --candidate");
            }
        }
        Commands::Report { candidate_id, format: _ } => {
            let db = Database::new("hirebridge.db").expect("Failed to open database");
            let claims = db.get_verified_claims(&candidate_id).expect("Failed to get claims");
            let stats = db.get_candidate_stats(&candidate_id).expect("Failed to get stats");
            println!("Candidate: {}", candidate_id);
            println!("Claims: {}", stats.0);
            println!("Pass: {}", stats.1);
            println!("Fail: {}", stats.2);
            for claim in claims {
                println!("  {}: {}", claim.id, claim.claim_text);
            }
        }
        Commands::Tamper { claim_id } => {
            let detector = TamperDetector::new();
            let db = Database::new("hirebridge.db").expect("Failed to open database");
            let claims = db.get_verified_claims(&claim_id).expect("Failed to get claims");
            if claims.is_empty() {
                eprintln!("Claim not found: {}", claim_id);
                return;
            }
            let result = detector.run_tamper_suite(&claims[0]);
            println!("Tamper suite: {}/{} mutations caught", result.caught, result.total);
        }
        Commands::Compare { candidate_a, candidate_b } => {
            let db = Database::new("hirebridge.db").expect("Failed to open database");
            let stats_a = db.get_candidate_stats(&candidate_a).expect("Failed to get stats");
            let stats_b = db.get_candidate_stats(&candidate_b).expect("Failed to get stats");
            println!("Candidate A ({}): {} claims, {} pass, {} fail", candidate_a, stats_a.0, stats_a.1, stats_a.2);
            println!("Candidate B ({}): {} claims, {} pass, {} fail", candidate_b, stats_b.0, stats_b.1, stats_b.2);
        }
        Commands::Task { title, description, repo, setup, verify, timeout, candidate, outcome, expected, exit_code } => {
            let harness = Harness::new("./sandbox", timeout);
            let expected_outcome = match outcome.as_str() {
                "exit" => ExpectedOutcome::ExitCode(exit_code),
                "both" => ExpectedOutcome::Both(expected.unwrap_or_default(), exit_code),
                _ => ExpectedOutcome::Pass(expected.unwrap_or_default()),
            };
            let task = Task {
                id: format!("task-{}", chrono::Utc::now().timestamp()),
                title,
                description,
                repo_url: repo,
                setup_commands: setup,
                verify_commands: verify,
                timeout_seconds: timeout,
                expected_outcome,
            };
            let result = harness.execute_task(&task, &candidate);
            println!("Task result: {:?}", result.status);
            println!("Duration: {}s", result.duration_seconds);
            for e in &result.evidence {
                println!("  {}: exit {} — {}", e.command, e.exit_code, e.output.trim());
            }
        }
        Commands::Team { challenge, candidates } => {
            let mut builder = TeamBuilder::new();
            for candidate_id in &candidates {
                let member = TeamMember {
                    candidate_id: candidate_id.clone(),
                    name: candidate_id.clone(),
                    role: "Contributor".to_string(),
                    skills: vec!["Rust".to_string(), "TypeScript".to_string()],
                    communication_style: CommunicationStyle::Direct,
                    leadership_score: 5,
                    collaboration_score: 7,
                };
                builder.add_member(member);
            }
            let challenge_obj = TeamChallenge {
                id: challenge.clone(),
                title: "Team Challenge".to_string(),
                description: "Collaborative task".to_string(),
                required_skills: vec!["Rust".to_string()],
                min_members: 2,
                max_members: 4,
                duration_minutes: 60,
            };
            builder.add_challenge(challenge_obj);
            let challenge_id = challenge.clone();
            let team = builder.form_team(&challenge_id).expect("Failed to form team");
            println!("Team formed: {} members", team.members.len());
            println!("Compatibility score: {:.2}", team.compatibility_score);
            for (id, role) in &team.role_assignment {
                println!("  {}: {}", id, role);
            }
        }
        Commands::Assess { candidate, collaboration, leadership, style } => {
            let style_str = style.clone();
            let style_enum = match style_str.as_str() {
                "analytical" => CommunicationStyle::Analytical,
                "creative" => CommunicationStyle::Creative,
                "structured" => CommunicationStyle::Structured,
                "adaptive" => CommunicationStyle::Adaptive,
                _ => CommunicationStyle::Direct,
            };
            let _member = TeamMember {
                candidate_id: candidate.clone(),
                name: candidate.clone(),
                role: "Contributor".to_string(),
                skills: vec!["Rust".to_string(), "TypeScript".to_string()],
                communication_style: style_enum,
                leadership_score: leadership,
                collaboration_score: collaboration,
            };
            println!("Assessed: {}", candidate);
            println!("  Collaboration: {}", collaboration);
            println!("  Leadership: {}", leadership);
            println!("  Style: {:?}", style);
        }
        Commands::Mission { title, mtype, difficulty, candidate, output } => {
            let mut evaluator = MissionEvaluator::new();
            let mtype_enum = match mtype.as_str() {
                "crisis" => MissionType::CrisisManagement,
                "innovation" => MissionType::Innovation,
                "crossfunctional" => MissionType::CrossFunctional,
                _ => MissionType::StrategicPlanning,
            };
            let difficulty_enum = match difficulty.as_str() {
                "easy" => Difficulty::Easy,
                "hard" => Difficulty::Hard,
                "expert" => Difficulty::Expert,
                _ => Difficulty::Medium,
            };
            let mission_id = format!("mission-{}", chrono::Utc::now().timestamp());
            let mission = Mission {
                id: mission_id.clone(),
                title: title.clone(),
                description: format!("{:?} mission at {:?} difficulty", mtype_enum, difficulty_enum),
                mission_type: mtype_enum,
                difficulty: difficulty_enum,
                duration_minutes: 60,
                success_criteria: vec![title.clone()],
                failure_criteria: vec!["fail".to_string()],
            };
            evaluator.add_mission(mission);
            let result = evaluator.evaluate(&mission_id, &candidate, &output);
            println!("Mission result: {:?}", result.status);
            println!("Success criteria met: {:?}", result.success_criteria_met);
            println!("Failure criteria met: {:?}", result.failure_criteria_met);
            println!("Adaptive challenge triggered: {}", result.adaptive_challenge_triggered);
        }
        Commands::AssessCandidate { candidate, skills, role } => {
            let engine = ProfessionalDevEngine::new();
            let required_skills: Vec<SkillProficiency> = skills.iter().map(|s| {
                SkillProficiency {
                    name: s.clone(),
                    level: SkillLevel::Intermediate,
                    years_experience: 1.0,
                }
            }).collect();
            let profile = CompetencyProfile {
                candidate_id: candidate.clone(),
                skills: vec![
                    SkillProficiency { name: "Rust".to_string(), level: SkillLevel::Advanced, years_experience: 3.0 },
                    SkillProficiency { name: "TypeScript".to_string(), level: SkillLevel::Intermediate, years_experience: 2.0 },
                ],
                experience_years: 4.0,
                certifications: vec!["Rust Certified".to_string()],
                role_target: role.clone(),
            };
            let evidence = engine.assess_candidate(&profile, &required_skills);
            println!("Assessment for: {}", candidate);
            println!("Role target: {}", role);
            println!("Exit code: {}", evidence.exit_code);
            println!("{}", evidence.output);
        }
        Commands::LearningPlan { candidate, gaps } => {
            let engine = ProfessionalDevEngine::new();
            let gap_list: Vec<CompetencyGap> = gaps.iter().map(|g| CompetencyGap {
                skill_name: g.clone(),
                required_level: SkillLevel::Intermediate,
                current_level: SkillLevel::Beginner,
                gap_magnitude: 2,
            }).collect();
            let plan = engine.generate_learning_plan(&candidate, gap_list);
            println!("Learning plan for: {}", candidate);
            println!("Total duration: {} minutes", plan.total_duration_minutes);
            for resource in &plan.resources {
                println!("  [{}] {} ({} min)", match resource.resource_type {
                    ResourceType::Video => "video",
                    ResourceType::Article => "article",
                    ResourceType::Interactive => "interactive",
                    ResourceType::Quiz => "quiz",
                    ResourceType::Project => "project",
                }, resource.title, resource.duration_minutes);
            }
        }
        Commands::MentorMatch { candidate, skills } => {
            let engine = ProfessionalDevEngine::new();
            let gaps: Vec<CompetencyGap> = skills.iter().map(|s| CompetencyGap {
                skill_name: s.clone(),
                required_level: SkillLevel::Advanced,
                current_level: SkillLevel::Beginner,
                gap_magnitude: 2,
            }).collect();
            let matches = engine.find_mentors(&gaps);
            println!("Mentorship matches for: {}", candidate);
            for m in &matches {
                println!("  {} (score: {:.2})", m.mentor_name, m.match_score);
                println!("    Shared skills: {:?}", m.shared_skills);
                println!("    Focus: {}", m.recommended_focus);
            }
        }
        Commands::GitHubSync { candidate, repo, email } => {
            let client = GitHubClient::new();
            let db = Database::new("hirebridge.db").expect("Failed to open database");
            let claims = db.get_verified_claims(&candidate).expect("Failed to get claims");

            if claims.is_empty() {
                eprintln!("No claims found for candidate: {}", candidate);
                return;
            }

            let claim = &claims[0];
            let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            let commits = runtime.block_on(client.fetch_commits(&repo, &email))
                .expect("Failed to fetch commits");
            let prs = runtime.block_on(client.fetch_pr_history(&repo, &email))
                .expect("Failed to fetch PRs");

            let evidence = client.verify_repo_activity(claim, &commits);
            println!("GitHub sync for: {}", candidate);
            println!("Commits: {}", commits.len());
            println!("PRs: {}", prs.len());
            println!("Verification: {}", if evidence.exit_code == 0 { "PASS" } else { "FAIL" });
            for commit in &commits {
                println!("  {} — {}", commit.sha[..7].to_string(), commit.commit.message.lines().next().unwrap_or(""));
            }
        }
        Commands::Serve { addr } => {
            let db_path = "hirebridge.db".to_string();
            let addr: std::net::SocketAddr = addr.parse().expect("Invalid address");
            println!("HireBridge dashboard at http://{}", addr);
            let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            runtime.block_on(start_server(db_path, addr));
        }
    }
}
