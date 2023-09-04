use std::{process::ExitCode, env::Args, path::PathBuf};

use crate::config;

/// All ToolInst commands.
pub mod commands;

/// Executes actions depending on CLI arguments.
/// 
/// If termination is needed, it will return `Some()` with the proper `ExitCode`.
pub fn run_args(args: Vec<String>, config: &config::ToolInst) -> Option<ExitCode> {
    let mut i = 0;
    while let Some(arg) = args.get(i) {
        if arg.trim() == "--version" || arg.trim() == "-v" {
            println!("v0.1.0");
            return Some(ExitCode::from(0));
        }
        if arg.trim() == "--help" || arg.trim() == "-h" {
            println!("Usage:");
            println!("    toolinst [options] <commands>");
            println!("\nCommands:");
            println!("    list - Lists the contents of .compressed and .installer respectively.");
            println!("    comp [options] <file> - Moves a compressed file to the .compressed directory.");
            println!("\nOptions: ");
            println!("    --version | Prints the toolinst version.");
            println!("    --help | Prints this message.");
            
            return Some(ExitCode::from(0));
        }
        
        match arg.trim() {
            "list" => {commands::list(config)},
            "comp" => {
                let mut options = Vec::new();
                while let Some(option) = args.get(i + 1) {
                    if option.starts_with("--") {
                        options.push(option.clone());
                        i += 1;
                    } else {
                        i += 1;
                        break;
                    }
                }
                let path = PathBuf::from(args.get(i).unwrap_or(&String::new()));
                if let Err(err) = commands::comp(options, &path, &config) {
                    eprintln!("Err! {}", err.to_string());
                    return Some(ExitCode::from(1));
                }
            }
            _ => {
                eprintln!("Unknow command.");
                return Some(ExitCode::from(1));
            }
        }

        i += 1;
    }

    None
}
