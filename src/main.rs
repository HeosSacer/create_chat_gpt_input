use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

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
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <root_dir> <extension> <output_file> <separator>", args[0]);
        return;
    }

    let root_dir = &args[1];
    let extension = &args[2];
    let output_file = &args[3];
    let separator = &args[4];

    let file_paths = collect_files_with_extension(Path::new(root_dir), extension)
        .expect("Failed to collect files with specified extension");

    merge_files(&file_paths, output_file, separator)
        .expect("Failed to merge file contents");

    println!("Successfully merged {} files into {}.", file_paths.len(), output_file);
}
