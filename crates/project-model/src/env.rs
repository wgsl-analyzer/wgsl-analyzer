use base_db::Environment;
use paths::Utf8Path;
use toolchain::Tool;

use crate::ManifestPath;

pub(crate) fn inject_wesl_env(environment: &mut Environment) {
    environment.set("WESL", Tool::Wesl.path().to_string());
}
