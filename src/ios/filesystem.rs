use std::path::{PathBuf, Path};
use std::fs;
use tokio::process::Command;
use std::io;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum FileSystemError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to move file '{path}': {message}")]
    MoveError {
        path: PathBuf,
        message: String,
    },
}

pub async fn move_music_to_device<P: AsRef<Path>>(mountpoint: P) -> Result<String, FileSystemError> {
    let mut moved_files = vec![];

    for entry in fs::read_dir(&crate::common::constants::download_path())? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let output = Command::new("mv")
                .arg(&path)
                .arg(mountpoint.as_ref())
                .output()
                .await?;

            if output.status.success() {
                moved_files.push(path.file_name().unwrap().to_string_lossy().into_owned());
            } else {
                return Err(FileSystemError::MoveError {
                    path: path,
                    message: String::from_utf8_lossy(&output.stderr).to_string(),
                });
            }
        }
    }

    Ok(format!(
        "Moved {} files to {} ✅: {:?}",
        moved_files.len(),
        mountpoint.as_ref().display(),
        moved_files
    ))
}