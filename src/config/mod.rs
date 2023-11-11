use serde::{Deserialize, Serialize};

use crate::app::{TisqKeyboundAction, KeybindingsConfig, SnippetsConfig};

#[derive(Serialize, Deserialize)]
pub(crate) struct TisqConfig {
    pub(crate) keybindings: Option<KeybindingsConfig<TisqKeyboundAction>>,
    pub(crate) snippets: Option<SnippetsConfig>,
}

impl TisqConfig {
    pub fn read_or_create(files_root: &std::path::PathBuf) -> eyre::Result<TisqConfig> {
        let config_path = files_root.join("config.toml");

        // create file if not exists
        if !config_path.exists() {
            std::fs::write(&config_path, "")?;
        }

        let config = std::fs::read_to_string(config_path)?;
        let config: TisqConfig = toml::from_str(&config)?;
        Ok(config)
    }
}
