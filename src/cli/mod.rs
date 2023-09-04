use std::process::ExitCode;

use crate::config;

/// All ToolInst commands.
pub mod commands;

/// Executes actions depending on CLI arguments.
/// 
/// If termination is needed, it will return `Some()` with the proper `ExitCode`.
pub fn run_args(args: Vec<String>, config: &config::ToolInst) -> Option<ExitCode> {
    for arg in args {
        if arg.trim() == "--version" || arg.trim() == "-v" {
            println!("v0.1.0");
            return Some(ExitCode::from(0));
        }
        if arg.trim() == "--help" || arg.trim() == "-h" {
            println!("Usage:");
            println!("    toolinst [options] <commands>");
            println!("\nCommands:");
            println!("    list - Lists the contents of .compressed and .installer respectively.");
            println!("\nOptions: ");
            println!("    --version | Prints the toolinst version.");
            println!("    --help | Prints this message.");
            
            return Some(ExitCode::from(0));
        }
        
        match arg.trim() {
            "list" => commands::list(config),
            _ => {
                eprintln!("Unknow command.");
                return Some(ExitCode::from(1));
            }
        }
    }

    Some(ExitCode::from(0))
}