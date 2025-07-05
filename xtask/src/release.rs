mod changelog;

use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use anyhow::{Context as _, bail};
use directories::ProjectDirs;
use stdx::JodChild;
use xshell::{Shell, cmd};

use crate::{date_iso, flags, is_release_tag, project_root};

impl flags::Release {
    pub(crate) fn run(
        self,
        shell: &Shell,
    ) -> anyhow::Result<()> {
        if !self.dry_run {
            cmd!(shell, "git switch release").run()?;
            cmd!(shell, "git fetch upstream --tags --force").run()?;
            cmd!(shell, "git reset --hard tags/nightly").run()?;
            // The `release` branch sometimes has a couple of cherry-picked
            // commits for patch releases. If that is the case, just overwrite
            // it. Because we are setting `release` branch to an up-to-date `nightly`
            // tag, this should not be problematic in general.
            //
            // Note that, as we tag releases, we do not worry about "losing"
            // commits -- they will be kept alive by the tag. More generally, we
            // do not care about historic releases all that much, it is fine even
            // to delete old tags.
            cmd!(shell, "git push --force").run()?;
        }

        let website_root = project_root().join("../wgsl-analyzer.github.io");
        {
            let _dir = shell.push_dir(&website_root); // spellchecker:disable-line
            cmd!(shell, "git switch src").run()?; // spellchecker:disable-line
            cmd!(shell, "git pull").run()?;
        }
        let changelog_directory = website_root.join("./thisweek/_posts");

        let today = date_iso(shell)?;
        let commit = cmd!(shell, "git rev-parse HEAD").read()?;
        #[expect(clippy::as_conversions, reason = "no better helper method")]
        #[expect(clippy::cast_sign_loss, reason = "asserted to be in-range")]
        #[expect(clippy::cast_possible_truncation, reason = "asserted to be in-range")]
        let changelog_n = shell
            .read_dir(changelog_directory.as_path())?
            .into_iter()
            .filter_map(|path| {
                path.file_stem()
                    .map(|stem| stem.to_string_lossy().to_string())
            })
            .filter_map(|string| string.splitn(5, '-').last().map(|n| n.replace('-', ".")))
            .filter_map(|string| string.parse::<f32>().ok())
            .inspect(|n| assert!((0_f32..(usize::MAX as f32)).contains(&n.floor())))
            .map(|n| 1 + n.floor() as usize)
            .max()
            .unwrap_or_default();

        let tags = cmd!(shell, "git tag --list").read()?;
        let previous_tag = tags
            .lines()
            .filter(|line| is_release_tag(line))
            .next_back()
            .unwrap();

        let contents = changelog::get_changelog(shell, changelog_n, &commit, previous_tag, &today)?;
        let path = changelog_directory.join(format!("{today}-changelog-{changelog_n}.adoc"));
        shell.write_file(path, contents)?;

        Ok(())
    }
}
