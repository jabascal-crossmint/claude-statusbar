use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Cache {
    cache_dir: PathBuf,
}

impl Cache {
    pub fn new(git_dir: &str) -> Self {
        Self {
            cache_dir: Path::new(git_dir).join("statusbar"),
        }
    }

    pub fn get(&self, key: &str, ttl_secs: u64) -> Option<String> {
        let path = self.cache_dir.join(key);
        let ts_path = self.cache_dir.join(format!("{}.timestamp", key));

        let ts: i64 = fs::read_to_string(ts_path).ok()?.parse().ok()?;
        let age = current_timestamp() - ts;

        if age < ttl_secs as i64 {
            fs::read_to_string(path).ok()
        } else {
            None
        }
    }

    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        fs::create_dir_all(&self.cache_dir)?;
        fs::write(self.cache_dir.join(key), value)?;
        fs::write(
            self.cache_dir.join(format!("{}.timestamp", key)),
            current_timestamp().to_string(),
        )?;
        Ok(())
    }
}

fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
