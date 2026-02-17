use crate::cache::Cache;
use crate::git::exec;
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct PrInfo {
    pub url: String,
    pub number: u32,
}

#[derive(Deserialize)]
struct PrJson {
    url: String,
    number: u32,
}

/// Get PR URL and number for current branch with caching
pub fn get_pr(branch: &str, cwd: &str, cache: &Cache) -> PrInfo {
    let cache_key = format!("pr2-{}", branch);

    if let Some(cached) = cache.get(&cache_key, 60) {
        let mut parts = cached.splitn(2, ' ');
        if let (Some(num), Some(url)) = (parts.next(), parts.next()) {
            return PrInfo {
                url: url.to_string(),
                number: num.parse().unwrap_or(0),
            };
        }
    }

    let json = exec(
        "gh",
        &[
            "pr",
            "list",
            "--head",
            branch,
            "--json",
            "url,number",
            "--jq",
            "{url: (.[0].url // \"\"), number: (.[0].number // 0)}",
        ],
        Some(cwd),
    );

    let info: PrInfo = serde_json::from_str::<PrJson>(&json)
        .map(|j| PrInfo { url: j.url, number: j.number })
        .unwrap_or_default();

    let _ = cache.set(&cache_key, &format!("{} {}", info.number, info.url));
    info
}

/// Get overall CI check status: "pass", "fail", "pending", or ""
pub fn get_pr_overall_status(branch: &str, cwd: &str, cache: &Cache) -> String {
    let cache_key = format!("pr-overall-{}", branch);

    if let Some(cached) = cache.get(&cache_key, 30) {
        return cached;
    }

    let status = exec(
        "gh",
        &[
            "pr",
            "checks",
            "--json",
            "bucket",
            "--jq",
            "[.[] | .bucket] | if any(. == \"fail\") then \"fail\" elif any(. == \"pending\") then \"pending\" elif any(. == \"pass\") then \"pass\" else \"\" end",
        ],
        Some(cwd),
    );

    // gh returns quoted string from jq, strip quotes
    let status = status.trim().trim_matches('"').to_string();

    let _ = cache.set(&cache_key, &status);
    status
}
