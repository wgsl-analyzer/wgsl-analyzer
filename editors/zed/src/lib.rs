use std::fs;
use std::path::Path;
use zed::settings::LspSettings;
use zed_extension_api::{self as zed, LanguageServerId, Result, serde_json};

struct WgslExtension {
    cached_binary_path: Option<String>,
}

impl WgslExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String, String> {
        if let Some(path) = worktree.which("wgsl-analyzer") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path
            && fs::metadata(path).is_ok_and(|stat| stat.is_file())
        {
            return Ok(path.clone());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "wgsl-analyzer/wgsl-analyzer",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = match (platform, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "wgsl-analyzer-aarch64-apple-darwin.gz",
            (zed::Os::Mac, _) => "wgsl-analyzer-x86_64-apple-darwin.gz",
            (zed::Os::Linux, zed::Architecture::Aarch64) => {
                "wgsl-analyzer-aarch64-unknown-linux-gnu.gz"
            },
            (zed::Os::Linux, _) => "wgsl-analyzer-x86_64-unknown-linux-gnu.gz",
            (zed::Os::Windows, zed::Architecture::Aarch64) => {
                "wgsl-analyzer-aarch64-pc-windows-msvc.zip"
            },
            (zed::Os::Windows, _) => "wgsl-analyzer-x86_64-pc-windows-msvc.zip",
        };

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("wgsl-analyzer-{}", release.version);
        let bin_dir = format!("{}/bin", version_dir);

        fs::create_dir_all(&bin_dir)
            .map_err(|error| format!("failed to create directory '{bin_dir}': {error}"))?;

        let binary_name = if platform == zed::Os::Windows {
            "wgsl-analyzer.exe"
        } else {
            "wgsl-analyzer"
        };
        let binary_path = format!("{}/{}", bin_dir, binary_name);

        if !Path::new(&binary_path).exists() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let download_path = format!("{}/download", version_dir);

            zed::download_file(
                &asset.download_url,
                &download_path,
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::Gzip,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            if platform == zed::Os::Windows {
                let extracted_exe = match fs::read_dir(&download_path) {
                    Ok(entries) => {
                        let mut exe_path = None;
                        for entry in entries {
                            if let Ok(entry) = entry
                                && let Some(name) = entry.file_name().to_str()
                                && name.ends_with(".exe")
                            {
                                exe_path = Some(entry.path());
                                break;
                            }
                        }
                        exe_path.ok_or_else(|| "Could not find extracted executable".to_string())?
                    },
                    Err(e) => return Err(format!("Failed to read directory: {}", e)),
                };

                fs::rename(extracted_exe, &binary_path)
                    .map_err(|e| format!("Failed to move executable: {}", e))?;
            } else {
                fs::rename(download_path, &binary_path)
                    .map_err(|e| format!("Failed to move executable: {}", e))?;
            }

            zed::make_file_executable(&binary_path)?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir)
                    && entry
                        .file_name()
                        .to_str()
                        .is_some_and(|name| name.starts_with("wgsl-analyzer-"))
                {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for WgslExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>, String> {
        let settings = LspSettings::for_worktree("wgsl-analyzer", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "wgsl-analyzer": settings
        })))
    }
}

zed::register_extension!(WgslExtension);
