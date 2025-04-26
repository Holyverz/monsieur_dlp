use tokio::process::Command;
use thiserror::Error;
use std::io;

#[derive(Debug, Error)]
pub enum UsbMuxdError {
    #[error("Process failed to complete: {0}")]
    WaitError(#[from] io::Error),

    #[error("Output was not valid UTF-8: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

#[derive(Debug)]
pub enum UsbmuxdStatus {
    Running,
    Stopped,
}
//systemctl status usbmuxd.service
pub async fn check_usbmuxd_service_status() -> Result<UsbmuxdStatus, UsbMuxdError> {
    let output = Command::new("systemctl")
        .arg("is-active")
        .arg("usbmuxd.service")
        .output()
        .await?;

        match String::from_utf8(output.stdout) {
            Ok(stdout) => {
                match stdout.trim() {
                    "active" => Ok(UsbmuxdStatus::Running),
                    _ => Ok(UsbmuxdStatus::Stopped),
                }
            }
            Err(e) => {
                eprintln!("Failed to parse systemctl output as UTF-8: {}", e);
                Ok(UsbmuxdStatus::Stopped)
            }
        }
}