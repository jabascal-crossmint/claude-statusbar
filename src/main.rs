mod colors;
mod git;
mod input;
mod output;
mod transcript;

use clap::Parser;
use colors::*;
use git::exec;
use input::Input;
use output::build_output;
use transcript::get_context_pct;

use std::env;
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

    // Check if git repo
    if exec(
        "git",
        &["rev-parse", "--is-inside-work-tree"],
        Some(&current_dir),
    ) != "true"
    {
        let display = current_dir.replace(&env::var("HOME").unwrap_or_default(), "~");
        print!("{}{}{}", CYAN, display, RESET);
        return Ok(());
    }

    // Gather git info
    let branch = exec("git", &["branch", "--show-current"], Some(&current_dir));
    let git_dir = exec("git", &["rev-parse", "--git-common-dir"], Some(&current_dir));
    let is_worktree = git_dir.contains("/.git/worktrees/");

    // Context percentage
    let context_pct = input
        .transcript_path
        .as_ref()
        .and_then(|p| get_context_pct(p));

    // Build and print output
    let output = build_output(
        &current_dir,
        &branch,
        is_worktree,
        input
            .model
            .as_ref()
            .and_then(|m| m.display_name.as_deref()),
        context_pct.as_deref(),
    );

    print!("{}", output);

    Ok(())
}
