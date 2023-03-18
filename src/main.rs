use clap::Parser; // Use Parser instead of Clap
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Parser)]
#[clap(version = "1.0", author = "Your Name <your.email@example.com>", about = "Merge files with the specified extension")]
struct Opts {
    /// Root directory for searching files
    root_dir: String,

    /// File extension to search for
    extension: String,

    /// Output file to merge contents into
    output_file: String,

    /// Separator to use between merged file contents
    separator: String,
}

fn collect_files_with_extension(path: &Path, extension: &str) -> io::Result<Vec<String>> {
    let mut file_paths = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            file_paths.extend(collect_files_with_extension(&path, extension)?);
        } else if Some(extension) == path.extension().and_then(|ext| ext.to_str()) {
            file_paths.push(path.to_string_lossy().to_string());
        }
    }
    Ok(file_paths)
}

fn merge_files(file_paths: &[String], output_file: &str, separator: &str) -> io::Result<()> {
    let mut output = File::create(output_file)?;
    for file_path in file_paths {
        let mut input_file = File::open(file_path)?;
        let mut content = String::new();
        input_file.read_to_string(&mut content)?;

        writeln!(output, "{}\n{}", Path::new(file_path).file_name().unwrap().to_string_lossy(), content)?;
        writeln!(output, "{}", separator)?;
    }
    Ok(())
}

fn main() {
    let opts: Opts = Opts::parse();

    let file_paths = collect_files_with_extension(Path::new(&opts.root_dir), &opts.extension)
        .expect("Failed to collect files with specified extension");

    merge_files(&file_paths, &opts.output_file, &opts.separator)
        .expect("Failed to merge file contents");

    println!("Successfully merged {} files into {}.", file_paths.len(), opts.output_file);
}
