use std::{path::PathBuf, fs, io};

use crate::config;

/// Lists the contents of .compressed and .installer respectively.
pub fn list(config: &config::ToolInst) {
    println!("Compressed: ");
    if let Ok(entries) = config.compressed_dir.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("|   {}", entry.file_name().to_str().unwrap_or(&entry.file_name().to_string_lossy()));
            } else {
                eprintln!("\nFailed to read entry in .compresed directory.\n");
            }
        }
    } else {
        eprintln!("Failed to read .compressed directory.\n");
    }

    println!("\nInstaller: ");
    if let Ok(entries) = config.installer_dir.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("|   {}", entry.file_name().to_str().unwrap_or(&entry.file_name().to_string_lossy()));
            } else {
                eprintln!("\nFailed to read entry in .installer directory.\n");
            }
        }
    } else {
        eprintln!("Failed to read .installer directory.\n");
    }
}

/// Moves a compressed file to the .compressed directory.
pub fn comp(options: Vec<String>, file: &PathBuf, config: &config::ToolInst) -> io::Result<()> {
    let mut use_copy = false;

    for option in options {
        if option == "--help" {
            println!("Description: ");
            println!("    Moves a compressed file to the .compressed directory.");
            println!("\nUsage: ");
            println!("    ... comp [options] <file>");
            println!("\nOptions: ");
            println!("    --copy | Don't move, copy the file into .compressed");
            println!("    --help | Shows this message");
            return Ok(());
        }
        if option == "--copy" {
            use_copy = true;
        }
    }

    if file.is_file() && !use_copy {
        fs::rename(file, config.compressed_dir.join(file.file_name().unwrap()))?;
        println!("Done.\n{} -> {}", file.display(), config.compressed_dir.join(file.file_name().unwrap()).display());
        return Ok(());
    } else if use_copy {
        fs::copy(file, config.compressed_dir.join(file.file_name().unwrap()))?;
        println!("Done.\n{} -(copy)> {}", file.display(), config.compressed_dir.join(file.file_name().unwrap()).display());
        return Ok(());
    }

    Err(io::Error::new(io::ErrorKind::InvalidInput, "The file doesn't exist or doesn't seem to be a file."))
}

/// Moves an installer to the .installer directory.
pub fn inst(options: Vec<String>, file: &PathBuf, config: &config::ToolInst) -> io::Result<()> {
    let mut use_copy = false;

    for option in options {
        if option == "--help" {
            println!("Description: ");
            println!("    Moves an installer to the .installer directory.");
            println!("\nUsage: ");
            println!("    ... inst [options] <file>");
            println!("\nOptions: ");
            println!("    --copy | Don't move, copy the file into .installer");
            println!("    --help | Shows this message");
            return Ok(());
        }
        if option == "--copy" {
            use_copy = true;
        }
    }

    if file.is_file() && !use_copy {
        fs::rename(file, config.installer_dir.join(file.file_name().unwrap()))?;
        println!("Done.\n{} -> {}", file.display(), config.installer_dir.join(file.file_name().unwrap()).display());
        return Ok(());
    } else if use_copy {
        fs::copy(file, config.installer_dir.join(file.file_name().unwrap()))?;
        println!("Done.\n{} -(copy)> {}", file.display(), config.installer_dir.join(file.file_name().unwrap()).display());
        return Ok(());
    }

    Err(io::Error::new(io::ErrorKind::InvalidInput, "The file doesn't exist or doesn't seem to be a file."))
}
