use crate::flags;
use anyhow::bail;
use std::env::{self, var};
use xshell::{Shell, cmd};

impl flags::PublishReleaseNotes {
    pub(crate) fn run(
        self,
        shell: &Shell,
    ) -> anyhow::Result<()> {
        let mut markdown = shell.read_file(&self.changelog)?;
        if !markdown.starts_with("# Changelog") {
            bail!("changelog Markdown should start with `# Changelog`");
        }
        const NEWLINES: &str = "\n\n";
        let Some(index) = markdown.find(NEWLINES) else {
            bail!("missing newlines after changelog title");
        };
        markdown.replace_range(0..index + NEWLINES.len(), "");

        let file_name = check_file_name(self.changelog)?;
        let tag_name = &file_name[0..10];
        let original_changelog_url = create_original_changelog_url(&file_name);
        let additional_paragraph =
            format!("\nSee also the [changelog post]({original_changelog_url}).");
        markdown.push_str(&additional_paragraph);
        if self.dry_run {
            println!("{markdown}");
        } else {
            update_release(shell, tag_name, &markdown)?;
        }
        Ok(())
    }
}

fn check_file_name<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<String> {
    let file_name = path
        .as_ref()
        .file_name()
        .ok_or_else(|| anyhow::format_err!("file name is not specified as `changelog`"))?
        .to_string_lossy();

    let mut characters = file_name.chars();
    if file_name.len() >= 10
        && characters.next().unwrap().is_ascii_digit()
        && characters.next().unwrap().is_ascii_digit()
        && characters.next().unwrap().is_ascii_digit()
        && characters.next().unwrap().is_ascii_digit()
        && characters.next().unwrap() == '-'
        && characters.next().unwrap().is_ascii_digit()
        && characters.next().unwrap().is_ascii_digit()
        && characters.next().unwrap() == '-'
        && characters.next().unwrap().is_ascii_digit()
        && characters.next().unwrap().is_ascii_digit()
    {
        Ok(file_name.to_string())
    } else {
        bail!("unexpected file name format; no date information prefixed")
    }
}

fn create_original_changelog_url(file_name: &str) -> String {
    let year = &file_name[0..4];
    let month = &file_name[5..7];
    let day = &file_name[8..10];
    let mut stem = &file_name[11..];
    if let Some(stripped) = stem.strip_suffix(".md") {
        stem = stripped;
    }
    format!("https://wgsl-analyzer.github.io/thisweek/{year}/{month}/{day}/{stem}.html")
}

fn update_release(
    shell: &Shell,
    tag_name: &str,
    release_notes: &str,
) -> anyhow::Result<()> {
    let Ok(token) = var("GITHUB_TOKEN") else {
        bail!(
            "Please obtain a personal access token from https://github.com/settings/tokens and set the `GITHUB_TOKEN` environment variable."
        )
    };
    let accept = "Accept: application/vnd.github+json";
    let authorization = format!("Authorization: Bearer {token}");
    let api_version = "X-GitHub-Api-Version: 2022-11-28";
    let release_url = "https://api.github.com/repos/wgsl-analyzer/wgsl-analyzer/releases";

    let release_json = cmd!(
        shell,
        "curl -sf -H {accept} -H {authorization} -H {api_version} {release_url}/tags/{tag_name}"
    )
    .read()?;
    let release_id = cmd!(shell, "jq .id").stdin(release_json).read()?;

    let mut patch = String::new();
    // note: the GitHub API does not update the target commit if the tag already exists
    write_json::object(&mut patch)
        .string("tag_name", tag_name)
        .string("target_commitish", "main")
        .string("name", tag_name)
        .string("body", release_notes)
        .bool("draft", false)
        .bool("prerelease", false);
    let output = cmd!(
        shell,
        "curl -sf -X PATCH -H {accept} -H {authorization} -H {api_version} {release_url}/{release_id} -d {patch}"
    )
    .read()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_changelog_url_creation() {
        let input = "2025-02-??-changelog-0.md";
        let actual = create_original_changelog_url(input);
        let expected = "https://wgsl-analyzer.github.io/thisweek/2024/02/??/changelog-0.html";
        // TODO enable this when there is actually a changelog
        // assert_eq!(actual, expected);
    }
}
