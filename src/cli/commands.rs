use crate::config;

/// Lists the contents of .compressed and .installer respectively.
pub fn list(config: &config::ToolInst) {
    println!("Compressed: ");
    if let Ok(entries) = config.compressed_dir.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{}", entry.file_name().to_str().unwrap_or(&entry.file_name().to_string_lossy()));
            } else {
                eprintln!("\nFailed to read entry in .compresed directory.\n");
            }
        }
    } else {
        eprintln!("Failed to read .compressed directory.\n");
    }

    println!("Installer: ");
    if let Ok(entries) = config.installer_dir.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{}", entry.file_name().to_str().unwrap_or(&entry.file_name().to_string_lossy()));
            } else {
                eprintln!("\nFailed to read entry in .installer directory.\n");
            }
        }
    } else {
        eprintln!("Failed to read .installer directory.\n");
    }
}
