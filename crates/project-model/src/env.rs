use base_db::Environment;
use paths::Utf8Path;
use toolchain::Tool;

use crate::{ManifestPath, wesl_config_file::WeslConfigFile};

pub(crate) fn wesl_config_env(
    manifest: &ManifestPath,
    config: &Option<WeslConfigFile>,
) -> Environment {
    let mut env = Environment::default();
    let Some(serde_json::Value::Object(env_json)) = config.as_ref().and_then(|c| c.get("env"))
    else {
        return env;
    };

    // FIXME: The base here should be the parent of the `.wesl/config` file, not the manifest.
    // But wesl does not provide this information.
    let base = <_ as AsRef<Utf8Path>>::as_ref(manifest.parent());

    for (key, entry) in env_json {
        let serde_json::Value::Object(entry) = entry else {
            continue;
        };
        let Some(value) = entry.get("value").and_then(|v| v.as_str()) else {
            continue;
        };

        let value = if entry
            .get("relative")
            .and_then(|v| v.as_bool())
            .is_some_and(std::convert::identity)
        {
            base.join(value).to_string()
        } else {
            value.to_owned()
        };
        env.insert(key, value);
    }

    env
}

pub(crate) fn inject_wesl_env(environment: &mut Environment) {
    environment.set("WESL", Tool::Wesl.path().to_string());
}
