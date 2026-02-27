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
