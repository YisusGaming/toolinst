use std::{path::PathBuf, fs, process::ExitCode, env};
use home;

/// ToolInst module for configs.
pub mod config;
/// ToolInst CLI.
pub mod cli;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    let config_path = match home::home_dir() {
        Some(dir) => dir.join(".toolinstrc"),
        None => PathBuf::new(),
    };

    if !config_path.is_file() {
        eprintln!("Warning! Config file doesn't exist or doesn't seem to be a file.");
    }

    let config = match fs::read_to_string(config_path) {
        Ok(configs) => config::parse_config(&configs),
        Err(err) => {
            eprintln!("Error! Couldn't read configs.\nConfigurations are needed and must provide a value for INSTALL.\nIf you haven't done it yet, please create a .toolinstrc file in your home directory.");
            eprintln!("\nThe Error was: {}", err.to_string());
            return ExitCode::from(1);
        },
    };

    if let Some(exit_code) = cli::run_args(args, &config) {
        return exit_code;
    }

    ExitCode::from(0)
}
