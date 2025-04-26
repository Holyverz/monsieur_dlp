use std::env;
use std::path::{PathBuf, Path};

// The VLC APP_ID
pub const APP_BUNDLE_ID: &str = "org.videolan.vlc-ios";

pub fn youtube_songs_file() -> PathBuf {
    Path::new("youtube_files").join("ytb-songs.txt")
}

pub fn youtube_songs_historic_path() -> PathBuf {
    Path::new("youtube_files").join("ytb-songs-historic.txt")
}

/// The mounting path for the ios device
pub fn mounting_path() -> PathBuf {
    convert_path_string_to_pathbuf("~/VLC")
}

// The path where ytb-dlp download the songs
pub fn download_path() -> PathBuf {
    convert_path_string_to_pathbuf("~/Music/DLP/")
}

/// Convert given string to pathbuf with converting home character (~) to the actual emplacement
fn convert_path_string_to_pathbuf(path: &str) -> PathBuf {
    // Check if path starts with "~/"
    if let Some(stripped) = path.strip_prefix("~/") {
         // Get the user's home directory
        if let Some(home_dir) = env::var_os("HOME") {
            return PathBuf::from(home_dir).join(stripped);
        }
    }

    PathBuf::from(path) // fallback, in case it's not ~/
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn onvert_path_string_to_pathbuf_with_expand_home_directory() {
        let home = env::var("HOME").expect("HOME environment variable not set");

        let input = "~/VLC";
        let expected = PathBuf::from(home).join("VLC");
        let actual = convert_path_string_to_pathbuf(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn convert_path_string_to_pathbuf_with_non_tilde_path() {
        let input = "/tmp/test";
        let expected = PathBuf::from(input);
        let actual = convert_path_string_to_pathbuf(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn convert_path_string_to_pathbuf_with_tilde_without_slash() {
        let input = "~not-home";
        let expected = PathBuf::from("~not-home"); // Should not expand
        let actual = convert_path_string_to_pathbuf(input);

        assert_eq!(actual, expected);
    }
}
