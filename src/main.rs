mod colors;
mod git;
mod input;
mod output;
mod transcript;

use clap::Parser;
use colors::*;
use git::exec;
use input::Input;
use output::{build_output, GitInfo};
use transcript::get_turn_count;

use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "claude-statusbar")]
#[command(about = "Claude Code status bar generator")]
struct Args {}

fn main() -> anyhow::Result<()> {
    Args::parse();

    // Read stdin
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input: Input = serde_json::from_str(&buffer)?;

    let Some(current_dir) = input.workspace.and_then(|w| w.current_dir) else {
        print!("{}~{}", CYAN, RESET);
        return Ok(());
    };

    // Git info is optional: model/context/turns are shown everywhere
    let in_git = exec(
        "git",
        &["rev-parse", "--is-inside-work-tree"],
        Some(&current_dir),
    ) == "true";
    let git = in_git.then(|| {
        let branch = exec("git", &["branch", "--show-current"], Some(&current_dir));
        let git_dir = exec("git", &["rev-parse", "--git-common-dir"], Some(&current_dir));
        GitInfo {
            branch,
            is_worktree: git_dir.contains("/.git/worktrees/"),
        }
    });

    // Context percentage comes straight from Claude Code; turn count from the transcript
    let context_pct = input
        .context_window
        .as_ref()
        .and_then(|c| c.used_percentage)
        .map(|pct| {
            if pct >= 90.0 {
                format!("{:.1}", pct)
            } else {
                format!("{}", pct.round() as u32)
            }
        });
    let turn_count = input
        .transcript_path
        .as_ref()
        .and_then(|p| get_turn_count(p));

    // Build and print output
    let output = build_output(
        &current_dir,
        git.as_ref(),
        input
            .model
            .as_ref()
            .and_then(|m| m.display_name.as_deref()),
        context_pct.as_deref(),
        turn_count,
    );

    print!("{}", output);

    Ok(())
}
