use crate::youtube::song::*;
use std::fs::{self, File, OpenOptions};
use std::io::Result;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// Read ytb-songs.txt file and extract the lines
pub fn read_songs<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let file = match OpenOptions::new().read(true).open(&path) {
        Ok(f) => f,
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Create empty file if not found
            File::create(&path)?;
            println!("File not found, created empty file ({:?}).", path.as_ref());
            return Ok(Vec::new());
        }
        Err(e) => return Err(e),
    };

    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

pub fn add_failed_downloads<P: AsRef<Path>>(songs: &Vec<Song>, path: P) -> Result<()> {
    let temp_file_name = path.as_ref().with_extension("txt.temp");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&temp_file_name)?;

    for song in songs {
        writeln!(file, "{}", song)?;
    }
    fs::rename(temp_file_name, path)?;
    Ok(())
}

pub fn add_success_downloads<P: AsRef<Path>>(
    songs: &Vec<Song>,
    historic_file_name: P,
) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&historic_file_name)?;

    for song in songs {
        writeln!(file, "{}", song)?;
    }
    Ok(())
}

/// Convert the lines from the file into Song objects
pub fn serialize_file(lines: Vec<String>) -> Vec<Song> {
    let mut songs = Vec::new();

    for line in lines {
        let line = line.trim();

        if line.is_empty() {
            continue; // skip empty lines
        }

        let mut parts = line.splitn(3, '|');

        let url = match parts.next() {
            Some(u) => u,
            None => continue,
        };

        let artist = match parts.next() {
            Some(a) => a,
            None => continue,
        };

        let name = match parts.next() {
            Some(n) => n,
            None => continue,
        };

        songs.push(Song::new(
            url.to_string(),
            artist.to_string(),
            name.to_string(),
        ));
    }

    songs
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn serialize_file_from_good_strings() {
        let lines = vec![
            "https://www.rust-lang.org/|Rust|Lang".to_string(),
            "https://crates.io/|Crates|Io".to_string(),
        ];
        let songs = serialize_file(lines);

        assert_eq!(songs.len(), 2);

        assert_eq!(
            songs[0],
            Song::new(
                "https://www.rust-lang.org/".to_string(),
                "Rust".to_string(),
                "Lang".to_string()
            )
        );

        assert_eq!(
            songs[1],
            Song::new(
                "https://crates.io/".to_string(),
                "Crates".to_string(),
                "Io".to_string()
            )
        );
    }

    #[test]
    fn serialize_file_with_empty_artist_strings() {
        let lines = vec!["https://www.rust-lang.org/||Lang".to_string()];
        let songs = serialize_file(lines);

        assert_eq!(songs.len(), 1);

        assert_eq!(
            songs[0],
            Song::new(
                "https://www.rust-lang.org/".to_string(),
                "".to_string(),
                "Lang".to_string()
            )
        );
    }

    #[test]
    fn serialize_file_with_empty_url_strings() {
        let lines = vec!["|Rust|Lang".to_string()];
        let songs = serialize_file(lines);

        assert_eq!(songs.len(), 1);

        assert_eq!(
            songs[0],
            Song::new("".to_string(), "Rust".to_string(), "Lang".to_string())
        );
    }

    #[test]
    fn serialize_file_with_empty_name_strings() {
        let lines = vec!["https://www.rust-lang.org/|Rust|".to_string()];
        let songs = serialize_file(lines);

        assert_eq!(songs.len(), 1);

        assert_eq!(
            songs[0],
            Song::new(
                "https://www.rust-lang.org/".to_string(),
                "Rust".to_string(),
                "".to_string()
            )
        );
    }

    #[test]
    fn serialize_file_with_incomplete_strings() {
        let lines = vec!["https://www.rust-lang.org/|Rust".to_string()];
        let songs = serialize_file(lines);

        assert_eq!(songs.len(), 0);
    }

    #[test]
    fn add_success_downloads_writes_correct_data() {
        let songs = vec![
            Song::new("https://url1.com".into(), "Artist1".into(), "Title1".into()),
            Song::new("https://url2.com".into(), "Artist2".into(), "Title2".into()),
        ];

        let temp_file_name = PathBuf::from("test_add_success_downloads_writes_correct_data.txt");

        if temp_file_name.exists() {
            // Cleanup
            fs::remove_file(&temp_file_name).unwrap();
        }

        // Run the function using the test-specific file
        add_success_downloads(&songs, &temp_file_name).unwrap();

        // Read back the content
        let content = fs::read_to_string(&temp_file_name).unwrap();

        let expected = "https://url1.com|Artist1|Title1\nhttps://url2.com|Artist2|Title2\n";
        assert_eq!(content, expected);

        // Cleanup
        fs::remove_file(temp_file_name).unwrap();
    }

    #[test]
    fn add_failed_downloads_writes_correct_data() {
        let songs = vec![
            Song::new("https://url1.com".into(), "Artist1".into(), "Title1".into()),
            Song::new("https://url2.com".into(), "Artist2".into(), "Title2".into()),
        ];

        let test_file = PathBuf::from("test_add_failed_downloads_writes_correct_data.txt");

        if test_file.exists() {
            // Cleanup
            fs::remove_file(&test_file).unwrap();
        }

        // Run the function using the test-specific file
        add_failed_downloads(&songs, &test_file).unwrap();

        // Read back the content
        let content = fs::read_to_string(&test_file).unwrap();

        let expected = "https://url1.com|Artist1|Title1\nhttps://url2.com|Artist2|Title2\n";
        assert_eq!(content, expected);

        // Cleanup
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn read_songs_creates_file_if_not_found() {
        // Define a temp file path but don't create the file
        let file_path =
            Path::new("youtube_files").join("test_read_songs_creates_file_if_not_found.txt");

        if file_path.exists() {
            // Cleanup
            fs::remove_file(&file_path).unwrap();
        }

        assert!(!file_path.exists());

        let result = read_songs(&file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Vec::<String>::new());

        // File should now exist
        assert!(file_path.exists());

        // Cleanup
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn read_songs_from_existing_file() {
        // Define a temp file path but don't create the file
        let file_path = Path::new("youtube_files").join("test_read_songs_from_existing_file.txt");

        if file_path.exists() {
            // Cleanup
            fs::remove_file(&file_path).unwrap();
        }

        let mut file = File::create(&file_path).expect("failed to create the test file");
        writeln!(file, "www.url1.com|Artist1|Name1").unwrap();
        writeln!(file, "www.url2.com|Artist2|Name2").unwrap();

        let result = read_songs(&file_path);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec!["www.url1.com|Artist1|Name1", "www.url2.com|Artist2|Name2"]
        );

        // Cleanup
        fs::remove_file(file_path).unwrap();
    }
}
