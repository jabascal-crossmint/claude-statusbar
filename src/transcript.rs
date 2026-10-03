use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Count queries in the transcript — matches claude-spend's queryCount.
/// One query = one assistant API call with usage (includes tool-call continuations).
/// The "~33 turns" insight threshold is based on this same count.
pub fn get_turn_count(path: &str) -> Option<u32> {
    let file = File::open(path).ok()?;
    let count = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .filter(|line| {
            serde_json::from_str::<Value>(line)
                .ok()
                .map(|j| j["type"] == "assistant" && !j["message"]["usage"].is_null())
                .unwrap_or(false)
        })
        .count() as u32;
    if count == 0 { None } else { Some(count) }
}
