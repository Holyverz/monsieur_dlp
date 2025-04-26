use super::super::song::Song;
use tokio::process::Command;
// Download songs with yt-dlp by executiong the downloader:
// yt-dlp -x -f bestaudio --extract-audio --audio-format mp3 -o "~/Music/Rust/SONGNAME" "URL"
pub async fn download_song(song: &Song) -> Result<(), String> {
    let mut status = Command::new("yt-dlp")
        .arg("-x")
        .arg("-f")
        .arg("bestaudio")
        .arg("--extract-audio")
        .arg("--audio-format")
        .arg("mp3")
        .arg("--postprocessor-args")
        .arg(format!(
            "ffmpeg:-metadata artist='{}' -metadata title='{}'",
            song.artist, song.name
        ))
        .arg("-o")
        .arg(format!("{}{}", crate::common::constants::download_path().display(), song.name))
        .arg(&song.url)
        .spawn()
        .map_err(|e| format!("Failed to spawn process: {}", e))?;

    let status = status
        .wait()
        .await
        .map_err(|e| format!("Process failed unexpectedly: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "Download failed with status: {}",
            status.code().unwrap_or(-1)
        ))
    }
}
