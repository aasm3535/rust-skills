use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Serialize, Clone)]
pub struct HistoryEntry {
    pub code: String,
    pub stdout: String,
    pub stderr: String,
    pub timestamp: DateTime<Utc>,
}

pub struct History {
    pub entries: Vec<HistoryEntry>,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, entry: HistoryEntry) {
        self.entries.push(entry);
    }
}
