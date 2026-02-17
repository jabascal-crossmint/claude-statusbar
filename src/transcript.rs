use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Get context percentage from transcript (optimized: only reads last 50 lines)
pub fn get_context_pct(path: &str) -> Option<String> {
    let file = File::open(path).ok()?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let start = lines.len().saturating_sub(50);
    let mut latest_usage: Option<serde_json::Map<String, Value>> = None;
    let mut latest_ts = i64::MIN;

    for line in &lines[start..] {
        let Ok(json) = serde_json::from_str::<Value>(line) else {
            continue;
        };

        if json["message"]["role"] == "assistant" {
            if let Some(ts_val) = json.get("timestamp") {
                let ts = if let Some(ts_num) = ts_val.as_i64() {
                    ts_num
                } else if let Some(ts_str) = ts_val.as_str() {
                    ts_str.parse::<i64>().unwrap_or(0)
                } else {
                    continue;
                };

                if ts > latest_ts {
                    latest_ts = ts;
                    latest_usage = json["message"]["usage"].as_object().cloned();
                }
            }
        }
    }

    if let Some(usage) = latest_usage {
        let used = usage
            .get("input_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0)
            + usage
                .get("output_tokens")
                .and_then(|v| v.as_u64())
                .unwrap_or(0)
            + usage
                .get("cache_read_input_tokens")
                .and_then(|v| v.as_u64())
                .unwrap_or(0)
            + usage
                .get("cache_creation_input_tokens")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

        let pct = (used as f64 * 100.0) / 156000.0;
        let pct = pct.min(100.0);

        Some(if pct >= 90.0 {
            format!("{:.1}", pct)
        } else {
            format!("{}", pct.round() as u32)
        })
    } else {
        None
    }
}

/// Get session duration from first and last timestamps
pub fn get_session_duration(path: &str) -> Option<String> {
    let file = File::open(path).ok()?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    if lines.is_empty() {
        return None;
    }

    let mut first_ts = None;
    let mut last_ts = None;

    // Get first timestamp
    for line in &lines {
        if let Ok(json) = serde_json::from_str::<Value>(line) {
            if let Some(ts_val) = json.get("timestamp") {
                let ts = if let Some(ts_num) = ts_val.as_i64() {
                    ts_num
                } else if let Some(ts_str) = ts_val.as_str() {
                    ts_str.parse::<i64>().ok()?
                } else {
                    continue;
                };
                first_ts = Some(ts);
                break;
            }
        }
    }

    // Get last timestamp
    for line in lines.iter().rev() {
        if let Ok(json) = serde_json::from_str::<Value>(line) {
            if let Some(ts_val) = json.get("timestamp") {
                let ts = if let Some(ts_num) = ts_val.as_i64() {
                    ts_num
                } else if let Some(ts_str) = ts_val.as_str() {
                    ts_str.parse::<i64>().ok()?
                } else {
                    continue;
                };
                last_ts = Some(ts);
                break;
            }
        }
    }

    let (first, last) = (first_ts?, last_ts?);
    let duration_secs = last - first;

    if duration_secs < 60 {
        Some("<1m".to_string())
    } else {
        let minutes = duration_secs / 60;
        let hours = minutes / 60;
        let remaining_minutes = minutes % 60;

        if hours > 0 {
            if remaining_minutes > 0 {
                Some(format!("{}h {}m", hours, remaining_minutes))
            } else {
                Some(format!("{}h", hours))
            }
        } else {
            Some(format!("{}m", minutes))
        }
    }
}

/// Get first user message from transcript (skipping commands and caveats)
pub fn get_first_user_message(path: &str) -> Option<String> {
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().filter_map(Result::ok) {
        if let Ok(json) = serde_json::from_str::<Value>(&line) {
            if json["message"]["role"] == "user" {
                if let Some(content) = json["message"]["content"].as_str() {
                    // Skip commands and system caveats
                    if content.starts_with('/') || content.contains("<system_caveats>") {
                        continue;
                    }
                    return Some(content.to_string());
                }
            }
        }
    }

    None
}
