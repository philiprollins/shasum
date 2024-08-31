use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512, Digest};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read};
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};
use clap::{Arg, Command};

fn main() -> io::Result<()> {
    let args = Command::new("shasum")
        .version("1.0")
        .about("Hashes files with various algorithms")
        .arg(
            Arg::new("algorithm")
                .short('a')
                .long("algorithm")
                .value_parser(["1", "224", "256", "384", "512"])
                .default_value("256")
                .help("Hash algorithm: 1 (SHA-1), 224, 256 (default), 384, 512"),
        )
        .arg(
            Arg::new("binary_mode")
                .short('b')
                .long("binary")
                .action(clap::ArgAction::SetTrue)
                .help("Read files in binary mode (default on DOS/Windows)"),
        )
        .arg(
            Arg::new("text_mode")
                .short('t')
                .long("text")
                .action(clap::ArgAction::SetTrue)
                .help("Read files in text mode (default)"),
        )
        .arg(
            Arg::new("quiet_mode")
                .short('q')
                .long("quiet")
                .action(clap::ArgAction::SetTrue)
                .help("Disable progress bar"),
        )
        .arg(
            Arg::new("filename")
                .help("File to hash")
                .required(true)
                .index(1),
        )
        .get_matches();

    let selected_algorithm = args.get_one::<String>("algorithm").unwrap();
    let filename = args.get_one::<String>("filename").map(|s| s.as_str());
    let is_binary_mode = args.get_flag("binary_mode");
    let is_text_mode = args.get_flag("text_mode");
    let is_quiet_mode = args.get_flag("quiet_mode");

    if let Some(file_path) = filename {
        let path = check_file_path(file_path)?;

        let hash_result = match selected_algorithm.as_str() {
            "1" => compute_file_hash::<Sha1>(&path, is_binary_mode || is_text_mode, is_quiet_mode)?,
            "224" => compute_file_hash::<Sha224>(&path, is_binary_mode || is_text_mode, is_quiet_mode)?,
            "256" => compute_file_hash::<Sha256>(&path, is_binary_mode || is_text_mode, is_quiet_mode)?,
            "384" => compute_file_hash::<Sha384>(&path, is_binary_mode || is_text_mode, is_quiet_mode)?,
            "512" => compute_file_hash::<Sha512>(&path, is_binary_mode || is_text_mode, is_quiet_mode)?,
            _ => unreachable!("Invalid algorithm value"),
        };

        println!("{}  {}", hash_result, file_path);
    }

    Ok(())
}

fn check_file_path(filename: &str) -> io::Result<&Path> {
    let path = Path::new(filename);

    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "The file does not exist."));
    }

    if !path.is_file() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "The path is not a file."));
    }

    Ok(path)
}

fn compute_file_hash<D: Digest>(path: &Path, binary_mode: bool, quiet_mode: bool) -> io::Result<String> {
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

fn create_progress_bar(total_size: u64) -> ProgressBar {
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
