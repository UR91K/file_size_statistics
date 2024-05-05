use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::io::{self, BufRead};

fn scan_files_with_extensions(dir: &Path, extensions: &[&str], files: &mut Vec<(String, u64)>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_files_with_extensions(&path, extensions, files);
            } else if let Some(file_extension) = path.extension() {
                if extensions.iter().any(|&ext| file_extension == ext) {
                    if let Ok(metadata) = fs::metadata(&path) {
                        if metadata.is_file() {
                            if let Some(file_name) = path.file_name() {
                                if let Some(file_size) = metadata.len().checked_div(1024) {
                                    files.push((file_name.to_string_lossy().into_owned(), file_size));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut files: Vec<(String, u64)> = Vec::new();
    let mut file_extensions = String::new();
    let stdin = io::stdin();

    println!("Enter the file extensions to search for (comma-separated, e.g., 'als,txt'): ");
    stdin.lock().read_line(&mut file_extensions).expect("Failed to read input.");
    let file_extensions: Vec<&str> = file_extensions.split(',').map(|s| s.trim()).collect();

    let mut folder_paths: Vec<PathBuf> = Vec::new();
    loop {
        println!("Enter a folder path (press Enter to start scanning): ");
        let mut folder_path = String::new();
        stdin.lock().read_line(&mut folder_path).expect("Failed to read input.");
        let folder_path = folder_path.trim();

        if folder_path.is_empty() {
            break;
        }

        let path = Path::new(folder_path);
        if path.is_dir() {
            folder_paths.push(path.to_path_buf());
        } else {
            println!("Invalid folder path. Please enter a valid folder path.");
        }
    }

    for folder_path in &folder_paths {
        scan_files_with_extensions(folder_path, &file_extensions, &mut files);
    }

    if files.is_empty() {
        println!("No files found with the specified extensions in the specified folder paths.");
        return;
    }

    let total_size: u64 = files.iter().map(|(_, size)| *size).sum();
    let mean_size = total_size as f64 / files.len() as f64;

    let mut size_counts: HashMap<u64, usize> = HashMap::new();
    for (_, size) in &files {
        *size_counts.entry(*size).or_insert(0) += 1;
    }
    let mode_size = size_counts.iter().max_by_key(|&(_, count)| count).map(|(size, _)| *size).unwrap_or(0);

    let mut sizes: Vec<u64> = files.iter().map(|(_, size)| *size).collect();
    sizes.sort();
    let median_size = if sizes.len() % 2 == 0 {
        let mid = sizes.len() / 2;
        sizes[mid - 1].checked_add(sizes[mid]).map_or(0, |sum| sum / 2)
    } else {
        sizes[sizes.len() / 2]
    };

    let lowest_size = sizes.first().cloned().unwrap_or(0);
    let highest_size = sizes.last().cloned().unwrap_or(0);

    println!("Total Files Found: {}", files.len());
    println!("Total Size: {:.2} KB", total_size as f64 / 1024.0);
    println!("Mean Size: {:.2} KB", mean_size);
    println!("Mode Size: {} KB", mode_size);
    println!("Median Size: {} KB", median_size);
    println!("Lowest Size: {} KB", lowest_size);
    println!("Highest Size: {} KB", highest_size);

    println!("Press Enter to exit...");
    let _ = stdin.lock().read_line(&mut String::new());
}
