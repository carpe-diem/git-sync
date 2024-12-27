use crate::config::Config;
use chrono::Local;
use std::io;
use std::process::Command;

pub struct Sync {
    config: Config,
}

impl Sync {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn run(&self) -> io::Result<()> {
        println!("📂 Synchronizing directory: {}", self.config.directory_path);

        std::env::set_current_dir(&self.config.directory_path)?;

        if !self.is_git_repo()? {
            println!("\n🚀 Initializing git repository...");
            self.git_init()?;
            self.git_remote_add()?;
            println!("✅ Git repository initialized successfully!");
        }

        println!("\n📝 Changed files:");
        self.show_status()?;

        if !self.has_changes()? {
            println!("\n✨ Nothing to synchronize!");
            return Ok(());
        }

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let commit_message = format!("git-sync: {}", timestamp);

        println!("\n🔄 Adding changes...");
        self.git_add()?;

        println!("📦 Committing changes...");
        self.git_commit(&commit_message)?;

        println!("⬆️  Pushing to remote...");
        self.git_push()?;

        println!("\n✅ Synchronization complete!");
        Ok(())
    }

    fn show_status(&self) -> io::Result<()> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let status = String::from_utf8_lossy(&output.stdout);
        for line in status.lines() {
            println!("  {}", line);
        }
        Ok(())
    }

    fn has_changes(&self) -> io::Result<bool> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(!output.stdout.is_empty())
    }

    fn git_add(&self) -> io::Result<()> {
        let output = Command::new("git")
            .args(["add", "."])
            .output()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(io::Error::new(io::ErrorKind::Other, error.to_string()));
        }
        Ok(())
    }

    fn git_commit(&self, message: &str) -> io::Result<()> {
        let output = Command::new("git")
            .args(["commit", "-m", message])
            .output()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(io::Error::new(io::ErrorKind::Other, error.to_string()));
        }

        let result = String::from_utf8_lossy(&output.stdout);
        println!("  {}", result.trim());
        Ok(())
    }

    fn git_push(&self) -> io::Result<()> {
        let output = Command::new("git")
            .args(["push", "origin", "main"])
            .output()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(io::Error::new(io::ErrorKind::Other, error.to_string()));
        }

        let result = String::from_utf8_lossy(&output.stdout);
        if !result.trim().is_empty() {
            println!("  {}", result.trim());
        }
        Ok(())
    }

    fn is_git_repo(&self) -> io::Result<bool> {
        let output = Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .output()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(output.status.success())
    }

    fn git_init(&self) -> io::Result<()> {
        let output = Command::new("git")
            .args(["init"])
            .output()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(io::Error::new(io::ErrorKind::Other, error.to_string()));
        }

        println!("  {}", String::from_utf8_lossy(&output.stdout).trim());
        Ok(())
    }

    fn git_remote_add(&self) -> io::Result<()> {
        let _ = Command::new("git")
            .args(["remote", "remove", "origin"])
            .output();

        let repo_url = format!("https://github.com/{}.git", self.config.github_repo);
        let output = Command::new("git")
            .args(["remote", "add", "origin", &repo_url])
            .output()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(io::Error::new(io::ErrorKind::Other, error.to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    fn create_test_config(dir: &TempDir) -> Config {
        Config {
            github_token: "test_token".to_string(),
            github_repo: "test/repo".to_string(),
            directory_path: dir.path().to_string_lossy().to_string(),
        }
    }

    fn create_test_file(dir: &Path, name: &str, content: &str) -> io::Result<()> {
        fs::write(dir.join(name), content)
    }

    fn init_git_repo(dir: &Path) -> io::Result<()> {
        Command::new("git")
            .args(["init", "--initial-branch=main"])
            .current_dir(dir)
            .status()?;

        for &(key, value) in &[
            ("user.name", "Test User"),
            ("user.email", "test@example.com"),
            ("commit.gpgsign", "false"),
        ] {
            Command::new("git")
                .args(["config", "--local", key, value])
                .current_dir(dir)
                .status()?;
        }

        Ok(())
    }

    // #[test]
    // fn test_is_git_repo() -> io::Result<()> {
    //     let original_dir = std::env::current_dir()?;
    //     let temp_dir = TempDir::new()?;
    //     let config = create_test_config(&temp_dir);
    //     let sync = Sync::new(config);

    //     std::env::set_current_dir(temp_dir.path())?;
    //     assert!(!sync.is_git_repo()?);

    //     init_git_repo(temp_dir.path())?;
    //     assert!(sync.is_git_repo()?);

    //     std::env::set_current_dir(original_dir)?;
    //     Ok(())
    // }

    // #[test]
    // fn test_has_changes() -> io::Result<()> {
    //     let original_dir = std::env::current_dir()?;
    //     let temp_dir = TempDir::new()?;
    //     let config = create_test_config(&temp_dir);
    //     let sync = Sync::new(config);

    //     std::env::set_current_dir(temp_dir.path())?;
    //     init_git_repo(temp_dir.path())?;

    //     assert!(!sync.has_changes()?);
    //     create_test_file(temp_dir.path(), "test.txt", "test content")?;
    //     assert!(sync.has_changes()?);

    //     std::env::set_current_dir(original_dir)?;
    //     Ok(())
    // }

    // #[test]
    // fn test_git_add_and_commit() -> io::Result<()> {
    //     let original_dir = std::env::current_dir()?;
    //     let temp_dir = TempDir::new()?;
    //     let config = create_test_config(&temp_dir);
    //     let sync = Sync::new(config);

    //     std::env::set_current_dir(temp_dir.path())?;
    //     init_git_repo(temp_dir.path())?;
    //     create_test_file(temp_dir.path(), "test.txt", "test content")?;

    //     sync.git_add()?;

    //     let status = Command::new("git")
    //         .args(["status", "--porcelain"])
    //         .current_dir(temp_dir.path())
    //         .output()?;

    //     let status_output = String::from_utf8_lossy(&status.stdout);
    //     assert!(status_output.contains("A  test.txt"));

    //     sync.git_commit("Test commit")?;
    //     assert!(!sync.has_changes()?);

    //     std::env::set_current_dir(original_dir)?;
    //     Ok(())
    // }

    // #[test]
    // fn test_git_remote_add() -> io::Result<()> {
    //     let original_dir = std::env::current_dir()?;
    //     let temp_dir = TempDir::new()?;
    //     let config = create_test_config(&temp_dir);
    //     let sync = Sync::new(config);

    //     std::env::set_current_dir(temp_dir.path())?;
    //     init_git_repo(temp_dir.path())?;
    //     sync.git_remote_add()?;

    //     let remote = Command::new("git")
    //         .args(["remote", "-v"])
    //         .current_dir(temp_dir.path())
    //         .output()?;

    //     let remote_output = String::from_utf8_lossy(&remote.stdout);
    //     assert!(remote_output.contains("origin"));

    //     std::env::set_current_dir(original_dir)?;
    //     Ok(())
    // }

    #[test]
    fn test_show_status() -> io::Result<()> {
        let original_dir = std::env::current_dir()?;
        let temp_dir = TempDir::new()?;
        let config = create_test_config(&temp_dir);
        let sync = Sync::new(config);

        std::env::set_current_dir(temp_dir.path())?;
        init_git_repo(temp_dir.path())?;
        create_test_file(temp_dir.path(), "test1.txt", "content1")?;
        create_test_file(temp_dir.path(), "test2.txt", "content2")?;

        sync.show_status()?;

        std::env::set_current_dir(original_dir)?;
        Ok(())
    }
}
