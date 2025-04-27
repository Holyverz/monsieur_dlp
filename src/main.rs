use std::{path::PathBuf, process::exit};

use tokio::task::JoinSet;
mod youtube;
use youtube::*;

mod common;
mod ios;

#[tokio::main]
async fn main() {
    // Check if usbmuxd service is running
    if let Err(err) = ios::service::check_usbmuxd_service_status().await {
        eprintln!("Warning: Failed to check usbmuxd status: {}", err);
        exit(0);
    }

    let lines = youtube::filesystem::read_songs(common::constants::youtube_songs_file())
        .expect("failed to read songs");
    let songs = youtube::filesystem::serialize_file(lines);
    // Create a JoinSet to manage our concurrent tasks
    let mut set = JoinSet::new();

    for song in &songs {
        let song_clone = song.clone();
        set.spawn(async move {
            match downloader::download_song(&song_clone).await {
                Ok(_) => {
                    println!("✅ Downloaded: {}", song_clone.name);
                    Ok(song_clone)
                }
                Err(e) => {
                    eprintln!("❌ Failed to download {}: {}", song_clone.name, e);
                    Err(song_clone)
                }
            }
        });
    }

    // Wait for all tasks to complete
    let task_results = set.join_all().await;
    let (success, fails): (Vec<_>, Vec<_>) = task_results.into_iter().partition(|x| x.is_ok());
    let success: Vec<Song> = success.into_iter().map(|x| x.ok().unwrap()).collect();
    let fails: Vec<Song> = fails.into_iter().map(|x| x.err().unwrap()).collect();

    // Add failed songs back to the songs text file
    match youtube::filesystem::add_failed_downloads(&fails, common::constants::youtube_songs_file())
    {
        Ok(()) => print!("✅ Files saved!"),
        Err(e) => eprintln!("Files writting failed unexpectedly: {}", e),
    }

    // Add Songs to the historic.txt
    match youtube::filesystem::add_success_downloads(
        &success,
        common::constants::youtube_songs_historic_path(),
    ) {
        Ok(()) => print!("✅ Files saved!"),
        Err(e) => eprintln!("Files writting failed unexpectedly: {}", e),
    }

    // Pair device
    match ios::pairing::pair_device().await {
        Ok(output) => println!("Pairing successful ✅\n{}", output),
        Err(err) => {
            eprintln!("Pairing failed ❌: {:?}", err);
            exit(0);
        }
    }

    // Validate device
    match ios::pairing::validate_device().await {
        Ok(output) => println!("Device validation successful ✅\n{}", output),
        Err(err) => {
            eprintln!("Validation failed ❌: {:?}", err);
            exit(0);
        }
    }

    // Mount device
    match ios::mounting::mount_app(
        common::constants::APP_BUNDLE_ID,
        &common::constants::mounting_path(),
    )
    .await
    {
        Ok(output) => println!("{}", output),
        Err(err) => {
            eprintln!("Mounting failed ❌: {:?}", err);
            exit(0);
        }
    }

    // Move songs to device
    match ios::filesystem::move_music_to_device(&common::constants::mounting_path()).await {
        Ok(output) => println!("{}", output),
        Err(err) => {
            eprintln!("Moving songs failed ❌: {:?}", err);
            exit(0);
        }
    }

    // Unmount device
    match ios::mounting::unmount(&common::constants::mounting_path()).await {
        Ok(output) => println!("{}", output),
        Err(err) => {
            eprintln!("Unmounting failed ❌: {:?}", err);
            exit(0);
        }
    }
}
