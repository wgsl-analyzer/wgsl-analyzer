use std::thread;
use std::time::Duration;
use std::{collections::HashSet, path::Path};

use anyhow::{Context as _, Result};
use regex::Regex;
use reqwest::StatusCode;
use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use xshell::{Shell, cmd};

use crate::{flags::Changelog, project_root};

impl Changelog {
    pub(crate) fn run(
        &self,
        shell: &Shell,
    ) -> anyhow::Result<()> {
        let token = std::env::var("GITHUB_TOKEN").ok();

        let commits = git_log_commits(shell, self.since.as_deref())?;
        eprintln!("Found {} commits referencing a PR", commits.len());

        let client = build_client(token.as_deref())?;

        for commit in &commits {
            let pr = fetch_pr_with_retry(&client, commit.pr_number)
                .with_context(|| format!("fetching PR #{}", commit.pr_number))?;

            let skip = pr
                .labels
                .iter()
                .find(|label| label.name == CHORE_LABEL || label.name == DEPENDENCIES_LABEL);

            match skip {
                None => {
                    println!("{}", commit.log);
                },
                Some(reason) => {
                    eprintln!(
                        "Skipping commit {} (PR #{} \"{}\" is labelled {})",
                        &commit.hash[..8.min(commit.hash.len())],
                        pr.number,
                        pr.title,
                        reason.name,
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

/// Maximum number of attempts before giving up on a single API call.
const MAX_RETRIES: u32 = 6;
/// Base delay for exponential backoff (doubled each attempt).
const BACKOFF_BASE_SECS: u64 = 2;

#[derive(Debug, Deserialize)]
struct Label {
    name: String,
}

#[derive(Debug, Deserialize)]
struct PullRequest {
    number: u64,
    title: String,
    labels: Vec<Label>,
}

#[derive(Debug)]
struct Commit {
    hash: String,
    log: String,
    pr_number: u64,
}

/// Run `git log` and return every commit that contains a `(#NNN)` PR
/// reference, stopping (exclusive) at `since_hash` when provided.
fn git_log_commits(
    shell: &Shell,
    since_hash: Option<&str>,
) -> Result<Vec<Commit>> {
    let separator = "\x1f"; // ASCII Unit Separator - safe in commit messages
    let format = format!("{separator}%H%n%h %s%n%nAuthor: %an <%ae>%nDate:   %ad%n%n    %b%n");

    let output = cmd!(shell, "git log --pretty=format:{format}")
        .output()
        .context("git log failed")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git log failed: {stderr}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    parse_commits(&stdout, since_hash)
}

/// Split raw `git log` output into [`Commit`] values, stopping before any
/// commit whose full hash starts with `since_hash`.
fn parse_commits(
    raw: &str,
    since_hash: Option<&str>,
) -> Result<Vec<Commit>> {
    let pull_request_re = Regex::new(r"\(#(\d+)\)").unwrap();
    let separator = '\x1f';

    let mut commits = Vec::new();

    for chunk in raw.split(separator) {
        let chunk = chunk.trim();
        if chunk.is_empty() {
            continue;
        }

        let (hash_line, log) = chunk
            .split_once('\n')
            .context("malformed commit chunk - no newline after hash")?;

        let hash = hash_line.trim().to_owned();

        // Honor --since: stop when we reach the boundary commit (exclusive).
        if let Some(prefix) = since_hash
            && hash.starts_with(prefix)
        {
            break;
        }

        // Only keep commits that reference a PR.
        let log = log.trim_end().to_owned();
        let subject = log.lines().next().unwrap_or("");
        if let Some(capabilities) = pull_request_re.captures(subject) {
            let pr_number: u64 = capabilities[1].parse().context("parsing PR number")?;
            commits.push(Commit {
                hash,
                log,
                pr_number,
            });
        }
    }

    Ok(commits)
}

/// Fetch a pull request, retrying on rate-limit (429) and transient server
/// errors (5xx) with exponential backoff.
///
/// The `Retry-After` header is respected when present; otherwise the delay
/// doubles on each attempt starting from [`BACKOFF_BASE_SECS`] seconds.
fn fetch_pr_with_retry(
    client: &Client,
    pr_number: u64,
) -> Result<PullRequest> {
    let url = format!("https://api.github.com/repos/{REPO_OWNER}/{REPO_NAME}/pulls/{pr_number}");

    let mut attempt = 0_u32;

    loop {
        let response = client
            .get(&url)
            .send()
            .with_context(|| format!("GET {url}"))?;

        let status = response.status();

        match status {
            // Success.
            status_code if status_code.is_success() => {
                return response
                    .json::<PullRequest>()
                    .context("deserialising GitHub PR response");
            },

            // Rate limited (primary 429) or secondary rate limit (403 with a
            // Retry-After header set by GitHub).
            StatusCode::TOO_MANY_REQUESTS | StatusCode::FORBIDDEN if attempt < MAX_RETRIES => {
                let wait = retry_after_secs(&response).unwrap_or_else(|| backoff_secs(attempt));

                eprintln!(
                    "Rate limited on PR #{pr_number} (HTTP {status}). \
                     Attempt {}/{MAX_RETRIES}. Waiting {wait}s…",
                    attempt + 1,
                );

                thread::sleep(Duration::from_secs(wait));
                attempt += 1;
            },

            // Transient server error — also worth retrying.
            status_code if status_code.is_server_error() && attempt < MAX_RETRIES => {
                let wait = backoff_secs(attempt);
                eprintln!(
                    "Server error {status_code} on PR #{pr_number}. \
                     Attempt {}/{MAX_RETRIES}. Waiting {wait}s…",
                    attempt + 1,
                );
                thread::sleep(Duration::from_secs(wait));
                attempt += 1;
            },

            // Anything else, or retries exhausted.
            _ => {
                let body = response.text().unwrap_or_default();
                anyhow::bail!(
                    "GitHub API returned {status} for PR #{pr_number} \
                     after {attempt} attempt(s): {body}"
                );
            },
        }
    }
}

/// Read the `Retry-After` header and return it as seconds, if present and
/// parseable as an integer.
fn retry_after_secs(response: &Response) -> Option<u64> {
    response
        .headers()
        .get("Retry-After")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
}

/// Exponential backoff: 2s, 4s, 8s, 16s, 32s, 64s …
const fn backoff_secs(attempt: u32) -> u64 {
    BACKOFF_BASE_SECS * 2_u64.pow(attempt)
}

fn build_client(token: Option<&str>) -> Result<Client> {
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        reqwest::header::ACCEPT,
        "application/vnd.github+json".parse().unwrap(),
    );
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
    headers.insert(
        reqwest::header::USER_AGENT,
        format!("{REPO_OWNER}/{REPO_NAME}-pr-changelog")
            .parse()
            .unwrap(),
    );

    if let Some(tok) = token {
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {tok}").parse().unwrap(),
        );
    }

    Client::builder()
        .default_headers(headers)
        .build()
        .context("building HTTP client")
}
