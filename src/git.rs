use std::process::{Command, Stdio};

/// Execute a command and return its stdout as a trimmed string
pub fn exec(cmd: &str, args: &[&str], cwd: Option<&str>) -> String {
    let mut command = Command::new(cmd);
    command
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null());

    if let Some(dir) = cwd {
        command.current_dir(dir);
    }

    command
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// Parse git status --porcelain output into compact format
pub fn parse_git_status(output: &str) -> String {
    if output.is_empty() {
        return String::new();
    }

    let mut added = 0;
    let mut modified = 0;
    let mut deleted = 0;
    let mut untracked = 0;

    for line in output.lines() {
        if line.len() < 3 {
            continue;
        }

        let status = &line[0..2];
        match status {
            "??" => untracked += 1,
            _ => {
                let x = status.chars().next().unwrap();
                let y = status.chars().nth(1).unwrap();

                // Check index (staged) status
                match x {
                    'A' => added += 1,
                    'M' => modified += 1,
                    'D' => deleted += 1,
                    _ => {}
                }

                // Check working tree status
                match y {
                    'M' => {
                        if x != 'M' {
                            modified += 1;
                        }
                    }
                    'D' => {
                        if x != 'D' {
                            deleted += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let mut parts = Vec::new();
    if added > 0 {
        parts.push(format!("+{}", added));
    }
    if modified > 0 {
        parts.push(format!("~{}", modified));
    }
    if deleted > 0 {
        parts.push(format!("-{}", deleted));
    }
    if untracked > 0 {
        parts.push(format!("?{}", untracked));
    }

    if parts.is_empty() {
        String::new()
    } else {
        format!(" {}", parts.join(" "))
    }
}

/// Parse git diff --numstat output and calculate net line changes
pub fn parse_diff_stats(output: &str) -> String {
    if output.is_empty() {
        return String::new();
    }

    let mut total_added = 0;
    let mut total_deleted = 0;

    for line in output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let (Ok(added), Ok(deleted)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                total_added += added;
                total_deleted += deleted;
            }
        }
    }

    let net = total_added - total_deleted;
    if net == 0 {
        String::new()
    } else if net > 0 {
        format!(" Δ+{}", net)
    } else {
        format!(" Δ{}", net)
    }
}
