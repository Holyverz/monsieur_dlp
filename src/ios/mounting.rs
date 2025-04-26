use tokio::process::Command;
use std::io;
use thiserror::Error;
use std::path::Path;

#[derive(Debug, Error)]
pub enum MountingError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
}

//ifuse --documents org.videolan.vlc-ios VLC
pub async fn mount_app<P: AsRef<Path>>(bundle_id: &str, mountpoint: P) -> Result<String, MountingError> {
    _ = Command::new("ifuse")
    .arg("--documents")
    .arg(bundle_id)
    .arg(mountpoint.as_ref())
    .output()
    .await?;

    Ok(format!("Mounting {} {} ✅", bundle_id, mountpoint.as_ref().display()))
}

//fusermount -u /home/nra/VLC
pub async fn unmount<P: AsRef<Path>>(mountpoint: P) -> Result<String, MountingError> {
    _ = Command::new("fusermount")
    .arg("-u")
    .arg(mountpoint.as_ref())
    .output()
    .await?;

    Ok(format!("Unmounting {} ✅", mountpoint.as_ref().display()))
}