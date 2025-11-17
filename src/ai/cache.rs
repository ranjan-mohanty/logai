use crate::types::ErrorAnalysis;
use crate::Result;
use rusqlite::{params, Connection};
use std::path::PathBuf;

pub struct AnalysisCache {
    conn: Connection,
}

impl AnalysisCache {
    pub fn new() -> Result<Self> {
        let cache_dir = Self::cache_dir()?;
        std::fs::create_dir_all(&cache_dir)?;

        let db_path = cache_dir.join("cache.db");
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS analysis_cache (
                pattern TEXT PRIMARY KEY,
                provider TEXT NOT NULL,
                model TEXT NOT NULL,
                explanation TEXT NOT NULL,
                root_cause TEXT,
                suggestions TEXT NOT NULL,
                created_at INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    fn cache_dir() -> Result<PathBuf> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map_err(|_| anyhow::anyhow!("Could not determine home directory"))?;
        Ok(PathBuf::from(home).join(".logai").join("cache"))
    }

    pub fn get(&self, pattern: &str, provider: &str, model: &str) -> Result<Option<ErrorAnalysis>> {
        let mut stmt = self.conn.prepare(
            "SELECT explanation, root_cause, suggestions 
             FROM analysis_cache 
             WHERE pattern = ?1 AND provider = ?2 AND model = ?3",
        )?;

        let result = stmt.query_row(params![pattern, provider, model], |row| {
            let explanation: String = row.get(0)?;
            let root_cause: Option<String> = row.get(1)?;
            let suggestions_json: String = row.get(2)?;

            Ok((explanation, root_cause, suggestions_json))
        });

        match result {
            Ok((explanation, root_cause, suggestions_json)) => {
                let suggestions = serde_json::from_str(&suggestions_json).unwrap_or_default();

                Ok(Some(ErrorAnalysis {
                    explanation,
                    root_cause,
                    suggestions,
                    related_resources: vec![],
                }))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn set(
        &self,
        pattern: &str,
        provider: &str,
        model: &str,
        analysis: &ErrorAnalysis,
    ) -> Result<()> {
        let suggestions_json = serde_json::to_string(&analysis.suggestions)?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        self.conn.execute(
            "INSERT OR REPLACE INTO analysis_cache 
             (pattern, provider, model, explanation, root_cause, suggestions, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                pattern,
                provider,
                model,
                &analysis.explanation,
                &analysis.root_cause,
                suggestions_json,
                now
            ],
        )?;

        Ok(())
    }

    pub fn clear_old(&self, days: i64) -> Result<usize> {
        let cutoff = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64
            - (days * 86400);

        let deleted = self.conn.execute(
            "DELETE FROM analysis_cache WHERE created_at < ?1",
            params![cutoff],
        )?;

        Ok(deleted)
    }
}
