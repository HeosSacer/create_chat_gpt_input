# Create ChatGPT Input

This Rust project is a command-line utility that searches for files with a specified extension in a directory and its subdirectories, and concatenates their contents into a single output file. The contents of each file are separated by a specified symbol and the file name.

## Usage

Run the compiled binary with the required arguments:

```bash
./create_chat_gpt_input /path/to/root-directory txt output.txt "====="
```

## Requirements

- Rust programming language (You can install it from [https://www.rust-lang.org/](https://www.rust-lang.org/))

## Building the Project

To build the project, run the following command:

```bash
cargo build --release
```

This will create an optimized binary at target/release/merge_files.

Replace /path/to/root-directory with the path to the directory where you want to start searching for files, txt with the desired file extension (without the dot), output.txt with the desired output file name, and ===== with the separator you want to use.

### Arguments

- `root_dir`: Root directory to start searching for files.
- `extension`: File extension to filter the files (without the dot).
- `output_file`: Output file to store the merged contents.
- `separator`: Separator symbol between file contents.
