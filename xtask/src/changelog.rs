use std::thread;
use std::time::Duration;

use anyhow::{Context as _, Result};
use xshell::{Shell, cmd};

use crate::flags::Changelog;

impl Changelog {
    pub(crate) fn run(
        &self,
        shell: &Shell,
    ) -> anyhow::Result<()> {
        let commits = git_log_commits(shell, self.since.as_deref())?;
        eprintln!("Found {} commits referencing a PR", commits.len());

        for commit in &commits {
            let labels = fetch_labels_with_retry(shell, commit.pr_number)
                .with_context(|| format!("fetching PR #{}", commit.pr_number))?;

            let skip = labels.iter().find(|name| {
                name.as_str() == CHORE_LABEL
                    || name.as_str() == DEPENDENCIES_LABEL
                    || name.as_str() == DOCUMENTATION_LABEL
            });

            match skip {
                None => {
                    println!("- {}", commit.log);
                },
                Some(reason) => {
                    eprintln!(
                        r#"Skipping #{} "{}" with label {reason}"#,
                        commit.pr_number, commit.log,
                    );
                },
            }
        }
        Ok(())
    }
}

const REPO_OWNER: &str = "wgsl-analyzer";
const REPO_NAME: &str = "wgsl-analyzer";
const CHORE_LABEL: &str = "C-Chore";
const DEPENDENCIES_LABEL: &str = "C-Dependencies";
const DOCUMENTATION_LABEL: &str = "C-Documentation";

/// Maximum number of attempts before giving up on a single API call.
const MAX_RETRIES: u32 = 6;
/// Base delay for exponential backoff (doubled each attempt).
const BACKOFF_BASE_SECS: u64 = 2;

#[derive(Debug)]
struct Change {
    log: String,
    pr_number: u16,
}

/// Fetch a pull request via `gh api`, retrying on failure with exponential
/// backoff. Authentication is handled by the `gh` CLI.
fn fetch_labels_with_retry(
    shell: &Shell,
    pr_number: u16,
) -> Result<Vec<String>> {
    let endpoint = format!("/repos/{REPO_OWNER}/{REPO_NAME}/issues/{pr_number}/labels");

    let mut attempt = 0_u32;

    loop {
        let command = cmd!(shell, "gh api {endpoint}")
            .args(["-H", "Accept: application/vnd.github+json"])
            .args(["-H", "X-GitHub-Api-Version: 2026-03-10"])
            .args(["--jq", ".[].name"]);
        let result = command.output().context("spawning gh")?;

        if result.status.success() {
            return Ok(String::from_utf8_lossy(&result.stdout)
                .lines()
                .map(ToOwned::to_owned)
                .collect());
        }

        if attempt >= MAX_RETRIES {
            let stderr = String::from_utf8_lossy(&result.stderr);
            anyhow::bail!("gh api failed for PR #{pr_number} after {attempt} attempt(s): {stderr}");
        }

        let wait = backoff_secs(attempt);
        let stderr = String::from_utf8_lossy(&result.stderr);
        eprintln!(
            "gh api failed for PR #{pr_number} (attempt {}/{MAX_RETRIES}): {stderr}\
             \nWaiting {wait}s…",
            attempt + 1,
        );
        thread::sleep(Duration::from_secs(wait));
        attempt += 1;
    }
}

fn git_log_commits(
    shell: &Shell,
    since_hash: Option<&str>,
) -> Result<Vec<Change>> {
    let since = if let Some(hash) = since_hash {
        format!("^{hash}")
    } else {
        String::new()
    };

    let output = cmd!(shell, "git log HEAD {since} --pretty=format:'%s'")
        .output()
        .context("running git log")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git log failed: {stderr}");
    }
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    parse_commits(&stdout)
}

fn parse_commits(raw: &str) -> Result<Vec<Change>> {
    let mut commits = Vec::new();

    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        const OPENER: &str = "(#";
        if let Some(pr_start) = line.rfind(OPENER)
            && let Some(pr_end) = line.rfind(')')
            && let pr_str = &line[pr_start + OPENER.len()..pr_end]
            && (pr_str.chars().all(|character| character.is_ascii_digit()) || pr_str.is_empty())
        {
            let pr_number: u16 = pr_str.parse().context("parsing PR number")?;
            let markdown_link = format!(
                "[(#{pr_number})](https://github.com/{REPO_OWNER}/{REPO_NAME}/pull/{pr_number})"
            );
            let log = format!("{}{markdown_link}", &line[..pr_start]);
            commits.push(Change { log, pr_number });
        } else {
            eprintln!("no PR for change {line}");
        }
    }

    Ok(commits)
}

/// Exponential backoff: 2 s, 4 s, 8 s, 16 s, 32 s, 64 s …
const fn backoff_secs(attempt: u32) -> u64 {
    BACKOFF_BASE_SECS * 2_u64.pow(attempt)
}
