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
        sh: &Shell,
    ) -> anyhow::Result<()> {
        if !self.dry_run {
            cmd!(sh, "git switch release").run()?;
            cmd!(sh, "git fetch upstream --tags --force").run()?;
            cmd!(sh, "git reset --hard tags/nightly").run()?;
            // The `release` branch sometimes has a couple of cherry-picked
            // commits for patch releases. If that is the case, just overwrite
            // it. Because we are setting `release` branch to an up-to-date `nightly`
            // tag, this should not be problematic in general.
            //
            // Note that, as we tag releases, we do not worry about "losing"
            // commits -- they will be kept alive by the tag. More generally, we
            // do not care about historic releases all that much, it is fine even
            // to delete old tags.
            cmd!(sh, "git push --force").run()?;
        }

        let website_root = project_root().join("../wgsl-analyzer.github.io");
        {
            let _dir = sh.push_dir(&website_root);
            cmd!(sh, "git switch src").run()?;
            cmd!(sh, "git pull").run()?;
        }
        let changelog_dir = website_root.join("./thisweek/_posts");

        let today = date_iso(sh)?;
        let commit = cmd!(sh, "git rev-parse HEAD").read()?;
        #[expect(clippy::as_conversions, reason = "intended")]
        #[expect(clippy::cast_sign_loss, reason = "intended")]
        let changelog_n = sh
            .read_dir(changelog_dir.as_path())?
            .into_iter()
            .filter_map(|path| {
                path.file_stem()
                    .map(|stem| stem.to_string_lossy().to_string())
            })
            .filter_map(|string| string.splitn(5, '-').last().map(|n| n.replace('-', ".")))
            .filter_map(|string| string.parse::<f32>().ok())
            .map(|n| 1 + n.floor() as usize)
            .max()
            .unwrap_or_default();

        let tags = cmd!(sh, "git tag --list").read()?;
        let prev_tag = tags
            .lines()
            .filter(|line| is_release_tag(line))
            .next_back()
            .unwrap();

        let contents = changelog::get_changelog(sh, changelog_n, &commit, prev_tag, &today)?;
        let path = changelog_dir.join(format!("{today}-changelog-{changelog_n}.adoc"));
        sh.write_file(path, contents)?;

        Ok(())
    }
}
