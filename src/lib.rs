use sha2::Digest;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read};
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};


pub fn check_file_path(filename: &str) -> io::Result<&Path> {
    let path = Path::new(filename);

    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "The file does not exist."));
    }

    if !path.is_file() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "The path is not a file."));
    }

    Ok(path)
}

pub fn compute_file_hash<D: Digest>(path: &Path, binary_mode: bool, quiet_mode: bool) -> io::Result<String> {
    let file = if binary_mode {
        OpenOptions::new().read(true).open(path)?
    } else {
        File::open(path)?
    };
    let metadata = file.metadata()?;
    let file_size = metadata.len();
    let mut reader = BufReader::new(file);
    let mut hasher = D::new();
    let mut buffer = [0; 8192];

    let progress_bar = if quiet_mode {
        None
    } else {
        Some(create_progress_bar(file_size))
    };

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);

        if let Some(pb) = &progress_bar {
            pb.inc(bytes_read as u64);
        }
    }

    if let Some(pb) = &progress_bar {
        pb.finish_with_message("Completed!");
    }

    let result = hasher.finalize();
    Ok(hex::encode(result))
}

pub fn create_progress_bar(total_size: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );
    progress_bar.set_message("Hashing...");

    progress_bar
}
