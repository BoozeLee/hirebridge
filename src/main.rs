mod claim;
mod verify;
mod tamper;
mod db;

use clap::{Parser, Subcommand};
use hirebridge::claim::{Claim, VerificationStatus};
use hirebridge::db::Database;
use hirebridge::verify::Verifier;
use hirebridge::tamper::TamperDetector;

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
        Commands::Verify { claim_id, clean_clone } => {
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
        Commands::Claims { candidate, status } => {
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
        Commands::Report { candidate_id, format } => {
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
    }
}
