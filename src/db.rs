use rusqlite::{Connection, Result, params};
use crate::claim::{Claim, VerificationStatus, Evidence};
use serde_json;
use chrono::{DateTime, Utc};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS claims (
                id TEXT PRIMARY KEY,
                candidate_id TEXT NOT NULL,
                claim_text TEXT NOT NULL,
                repo_url TEXT,
                status TEXT NOT NULL,
                evidence TEXT,
                checksum TEXT NOT NULL,
                created_at TEXT DEFAULT (datetime('now'))
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS candidates (
                id TEXT PRIMARY KEY,
                name TEXT,
                email TEXT,
                claims_count INTEGER DEFAULT 0,
                pass_count INTEGER DEFAULT 0,
                fail_count INTEGER DEFAULT 0,
                created_at TEXT DEFAULT (datetime('now'))
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn insert_claim(&self, claim: &Claim) -> Result<()> {
        let evidence_json = serde_json::to_string(&claim.evidence).unwrap_or_default();
        self.conn.execute(
            "INSERT INTO claims (id, candidate_id, claim_text, repo_url, status, evidence, checksum, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                claim.id,
                claim.candidate_id,
                claim.claim_text,
                claim.repo_url,
                claim.status.to_string(),
                evidence_json,
                claim.checksum,
                claim.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn get_verified_claims(&self, candidate_id: &str) -> Result<Vec<Claim>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM claims WHERE candidate_id = ?1 AND status = 'Pass'"
        )?;
        let claims = stmt.query_map([candidate_id], |row| {
            let evidence_str: String = row.get(5)?;
            let evidence: Vec<Evidence> = serde_json::from_str(&evidence_str).unwrap_or_default();
            let created_at_str: String = row.get(7)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or(Utc::now());
            Ok(Claim {
                id: row.get(0)?,
                candidate_id: row.get(1)?,
                claim_text: row.get(2)?,
                repo_url: row.get(3)?,
                code_snippet: None,
                verification_commands: Vec::new(),
                status: VerificationStatus::Pass,
                evidence,
                checksum: row.get(6)?,
                created_at,
            })
        })?.collect::<Result<Vec<_>>>()?;

        Ok(claims)
    }

    pub fn get_candidate_stats(&self, candidate_id: &str) -> Result<(i64, i64, i64)> {
        let mut stmt = self.conn.prepare(
            "SELECT claims_count, pass_count, fail_count FROM candidates WHERE id = ?1"
        )?;
        let stats = stmt.query_row([candidate_id], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?, row.get::<_, i64>(2)?))
        });

        match stats {
            Ok(s) => Ok(s),
            Err(_) => Ok((0, 0, 0)),
        }
    }

    pub fn get_all_candidates(&self) -> Result<Vec<serde_json::Value>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, claims_count, pass_count, fail_count FROM candidates"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, String>(0)?,
                "claims_count": row.get::<_, i64>(1)?,
                "pass_count": row.get::<_, i64>(2)?,
                "fail_count": row.get::<_, i64>(3)?,
                "pass_rate": if row.get::<_, i64>(1)? > 0 {
                    row.get::<_, i64>(2)? as f64 / row.get::<_, i64>(1)? as f64 * 100.0
                } else {
                    0.0
                }
            }))
        })?.collect::<Result<Vec<_>>>()?;
        Ok(rows)
    }
}
