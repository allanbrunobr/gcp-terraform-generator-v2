use anyhow::Result;
use std::process::Command;

pub struct CommandExecutor;

impl CommandExecutor {
    pub fn execute_gcloud(args: &[&str]) -> Result<String> {
        let output = Command::new("gcloud")
            .args(args)
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8(output.stdout)?)
    }
}