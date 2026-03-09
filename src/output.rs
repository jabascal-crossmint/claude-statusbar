use crate::colors::*;
use std::env;
use std::path::Path;

pub fn build_output(
    current_dir: &str,
    branch: &str,
    is_worktree: bool,
    model: Option<&str>,
    context_pct: Option<&str>,
    turn_count: Option<u32>,
) -> String {
    let display_dir = Path::new(current_dir)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(current_dir)
        .to_string();

    // Line 1: [model] context% 💬turns | directory
    let model_ctx = build_model_context(model, context_pct, turn_count);
    let line1 = format!("{}{}| {}{}{}", model_ctx, GRAY, CYAN, display_dir, RESET);

    // Line 2: ⎇ branch_name
    let branch_color = if is_worktree { MAGENTA } else { GREEN };
    let branch_display = if is_worktree {
        let dir_name = Path::new(current_dir)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if branch == dir_name {
            "↟".to_string()
        } else {
            format!("{}↟", branch)
        }
    } else {
        branch.to_string()
    };
    let line2 = format!("{}🌿 {}{}", branch_color, branch_display, RESET);

    vec![line1, line2].join("\n")
}

fn build_model_context(
    model: Option<&str>,
    context_pct: Option<&str>,
    turn_count: Option<u32>,
) -> String {
    let mut parts: Vec<String> = Vec::new();

    if let Some(model_name) = model {
        let abbrev = model_abbrev(model_name);
        parts.push(format!("{}[{}]{}", GRAY, abbrev, RESET));
    }

    if let Some(pct) = context_pct {
        let pct_num: f64 = pct.parse().unwrap_or(0.0);
        let pct_color = context_color(pct_num);
        parts.push(format!("{}{:.0}%{}", pct_color, pct_num, RESET));
    }

    if let Some(turns) = turn_count {
        let color = turn_color(turns);
        parts.push(format!("{}💬 {}{}", color, turns, RESET));
    }

    if parts.is_empty() {
        String::new()
    } else {
        format!("{} ", parts.join(" "))
    }
}

fn model_abbrev(model_name: &str) -> &'static str {
    let is_zai = env::var("ANTHROPIC_BASE_URL")
        .map(|u| u.contains("api.z.ai"))
        .unwrap_or(false);

    if is_zai {
        if model_name.contains("Opus") {
            "GLM"
        } else if model_name.contains("Sonnet") {
            "GPL-Air"
        } else if model_name.contains("Haiku") {
            "Haiku"
        } else {
            "?"
        }
    } else if model_name.contains("Opus") {
        "Opus"
    } else if model_name.contains("Sonnet") {
        "Sonnet"
    } else if model_name.contains("Haiku") {
        "Haiku"
    } else {
        "?"
    }
}

fn context_color(pct: f64) -> &'static str {
    if pct >= 90.0 {
        RED
    } else if pct >= 70.0 {
        ORANGE
    } else if pct >= 50.0 {
        YELLOW
    } else {
        GRAY
    }
}

/// Color based on turn count.
/// Derived from insight: "Use /clear after ~33 turns to save tokens"
/// green=<22, yellow=22-33, red=>33
fn turn_color(turns: u32) -> &'static str {
    if turns > 33 {
        RED
    } else if turns >= 22 {
        YELLOW
    } else {
        GREEN
    }
}
