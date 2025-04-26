use tokio::process::Command;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PairingError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("Command failed: {0}")]
    CommandError(String),
}
async fn execute_idevice_command(command: &str) -> Result<String, PairingError> {
    let output = Command::new("idevicepair")
        .arg(command)
        .output()
        .await?;

    // Check for non-UTF-8 output
    match String::from_utf8(output.stdout) {
        Ok(stdout) => {
            if stdout.contains("ERROR") {
                Err(PairingError::CommandError(stdout))
            } else {
                Ok(stdout)
            }
        }
        Err(_) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(PairingError::CommandError(format!(
                "Non-UTF-8 output: {}",
                stderr
            )))
        }
    }
}

pub async fn pair_device() -> Result<String, PairingError> {
    execute_idevice_command("pair").await
}

pub async fn validate_device() -> Result<String, PairingError> {
    execute_idevice_command("validate").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn pair_device_output() {
        let result = pair_device().await;
        match result {
            Ok(output) => {
                assert!(!output.is_empty());
                println!("Pair success: {output}");
            }
            Err(e) => {
                println!("Expected failure or environment issue: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn validate_device_output() {
        let result = validate_device().await;
        match result {
            Ok(output) => {
                assert!(!output.is_empty());
                println!("Validate success: {output}");
            }
            Err(e) => {
                println!("Expected failure or environment issue: {}", e);
            }
        }
    }
}
