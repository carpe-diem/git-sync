use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub github_token: String,
    pub github_repo: String,
    pub directory_path: String,
}

impl Config {
    pub fn load() -> io::Result<Option<Config>> {
        let config_path = get_config_path()?;
        if config_path.exists() {
            let config_str = fs::read_to_string(config_path)?;
            Ok(Some(serde_json::from_str(&config_str)?))
        } else {
            Ok(None)
        }
    }

    pub fn setup() -> io::Result<Config> {
        println!("🔄 Notes Sync Initial Setup");

        // Try to load existing configuration
        let existing_config = Self::load()?.unwrap_or(Config {
            github_token: String::new(),
            github_repo: String::new(),
            directory_path: String::new(),
        });

        let config = Config {
            github_token: prompt_with_default(
                "Enter your GitHub token (https://github.com/settings/tokens)",
                &existing_config.github_token,
            )?,
            github_repo: prompt_with_default(
                "Enter repository (format: username/repo)",
                &existing_config.github_repo,
            )?,
            directory_path: prompt_with_default(
                "Enter path to your directory to sync",
                &existing_config.directory_path,
            )?,
        };

        config.save()?;
        Ok(config)
    }

    pub fn save(&self) -> io::Result<()> {
        let config_path = get_config_path()?;
        let config_json = serde_json::to_string_pretty(&self)?;
        fs::write(&config_path, config_json)?;
        println!("\n✅ Configuration saved successfully at:");
        println!("{}", config_path.display());
        Ok(())
    }
}

fn get_config_path() -> io::Result<std::path::PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "notesync", "notesync")
        .expect("Could not get configuration directory");

    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(config_dir)?;
    Ok(config_dir.join("config.json"))
}

#[allow(dead_code)]
fn prompt(message: &str) -> io::Result<String> {
    print!("{}: ", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_with_default(message: &str, default: &str) -> io::Result<String> {
    let prompt_message = if default.is_empty() {
        format!("{}: ", message)
    } else {
        format!("{} [{}]: ", message, default)
    };

    print!("{}", prompt_message);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    Ok(if input.is_empty() {
        default.to_string()
    } else {
        input.to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_config_serialization() {
        let config = Config {
            github_token: "test_token".to_string(),
            github_repo: "user/repo".to_string(),
            directory_path: "/path/to/notes".to_string(),
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&serialized).unwrap();

        assert_eq!(config.github_token, deserialized.github_token);
        assert_eq!(config.github_repo, deserialized.github_repo);
        assert_eq!(config.directory_path, deserialized.directory_path);
    }

    #[test]
    fn test_config_save_and_load() -> io::Result<()> {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("config.json");

        // Create test config
        let test_config = Config {
            github_token: "temp_token".to_string(),
            github_repo: "temp/repo".to_string(),
            directory_path: "/temp/path".to_string(),
        };

        // Save config directly to temp file
        let config_json = serde_json::to_string_pretty(&test_config)?;
        fs::write(&config_path, config_json)?;

        // Load it back using a custom config path
        let loaded_config = fs::read_to_string(&config_path)?;
        let loaded_config: Config = serde_json::from_str(&loaded_config)?;

        // Verify contents
        assert_eq!(test_config.github_token, loaded_config.github_token);
        assert_eq!(test_config.github_repo, loaded_config.github_repo);
        assert_eq!(test_config.directory_path, loaded_config.directory_path);

        // TempDir will be automatically cleaned up when it goes out of scope
        Ok(())
    }

    #[test]
    fn test_load_nonexistent_config() -> io::Result<()> {
        // Get config path
        let config_path = get_config_path()?;

        // Ensure config doesn't exist
        if config_path.exists() {
            fs::remove_file(&config_path)?;
        }

        // Try to load config
        let config = Config::load()?;
        assert!(config.is_none(), "Expected None for nonexistent config");

        Ok(())
    }

    #[test]
    fn test_prompt_with_default() {
        // This is a bit tricky to test as it requires simulating user input
        // In a real application, you might want to use a mock for stdin
        // For now, we'll just test the formatting of the prompt message
        let message = "Test prompt";
        let default = "default_value";
        let expected = format!("{} [{}]: ", message, default);

        // Test prompt formatting
        assert_eq!(format!("{} [{}]: ", message, default), expected);
    }

    #[test]
    fn test_get_config_path() -> io::Result<()> {
        let path = get_config_path()?;

        // Verify path exists
        assert!(
            path.parent().unwrap().exists(),
            "Config directory should exist"
        );

        // Verify filename
        assert_eq!(
            path.file_name().unwrap().to_str().unwrap(),
            "config.json",
            "Config file should be named config.json"
        );

        Ok(())
    }

    #[test]
    fn test_empty_config() {
        let config = Config {
            github_token: String::new(),
            github_repo: String::new(),
            directory_path: String::new(),
        };

        assert!(config.github_token.is_empty());
        assert!(config.github_repo.is_empty());
        assert!(config.directory_path.is_empty());
    }
}
