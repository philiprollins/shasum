use clap::{Arg, Command};
use shasum::{check_file_path, compute_file_hash};
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use std::io;

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
