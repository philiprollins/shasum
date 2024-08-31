# shasum

`shasum` is a cross-platform command-line utility for computing and checking SHA message digests. It supports SHA-1, SHA-224, SHA-256, SHA-384, and SHA-512 algorithms.

## Features

- Supports multiple SHA algorithms (1, 224, 256, 384, 512)
- Cross-platform compatibility (Windows, macOS, Linux)
- Binary and text mode file reading
- Progress bar for visual feedback (can be disabled)
- Command-line interface with various options

## Installation

To install `shasum`, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can get them from [rustup.rs](https://rustup.rs/).

Once you have Rust and Cargo installed, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/philiprollins/shasum.git
   cd shasum
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The compiled binary will be available in the `target/release` directory.

## Usage

```
shasum [OPTIONS] <filename>
```

### Options

- `-a, --algorithm <ALGORITHM>`: Hash algorithm to use. Options are 1 (SHA-1), 224, 256 (default), 384, 512.
- `-b, --binary`: Read files in binary mode (default on DOS/Windows).
- `-t, --text`: Read files in text mode (default).
- `-q, --quiet`: Disable progress bar.
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

### Examples

1. Hash a file using the default algorithm (SHA-256):
   ```
   shasum myfile.txt
   ```

2. Use SHA-512 algorithm:
   ```
   shasum -a 512 myfile.txt
   ```

3. Hash a binary file:
   ```
   shasum -b myprogram.exe
   ```

4. Hash a file without displaying the progress bar:
   ```
   shasum -q largefile.iso
   ```

## Building for Other Platforms

`shasum` can be built for various platforms using Rust's cross-compilation features. Here are some examples:

- For Windows (from a Unix-like system):
  ```
  cargo build --release --target x86_64-pc-windows-gnu
  ```

- For macOS (from a Unix-like system):
  ```
  cargo build --release --target x86_64-apple-darwin
  ```

- For Linux (from a non-Linux system):
  ```
  cargo build --release --target x86_64-unknown-linux-gnu
  ```

Note: You may need to install the appropriate cross-compilation tools for your target platform.

## Dependencies

- `sha1`: SHA-1 implementation
- `sha2`: SHA-2 family implementations
- `indicatif`: Progress bar functionality
- `clap`: Command-line argument parsing
- `hex`: Hexadecimal encoding

## License

This project is released under the Unlicense. This means the software is dedicated to the public domain, and you can do whatever you want with it, without any restrictions. For more details, see the [UNLICENSE](UNLICENSE) file in the project repository or visit [unlicense.org](https://unlicense.org/).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Acknowledgments

- Inspired by the Unix/Linux `shasum` utility
- Built with Rust and its amazing ecosystem