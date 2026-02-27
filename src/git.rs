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
