use std::process::ExitCode;

/// Executes actions depending on CLI arguments.
/// 
/// If termination is needed, it will return `Some()` with the proper `ExitCode`.
pub fn run_args(args: Vec<String>) -> Option<ExitCode> {
    for arg in args {
        if arg.trim() == "--version" || arg.trim() == "-v" {
            println!("v0.1.0");
            return Some(ExitCode::from(0));
        }
        if arg.trim() == "--help" || arg.trim() == "-h" {
            println!("Usage:");
            println!("    toolinst [options] <commands>");
            println!("\nCommands:");
            // <- Commands here ->
            println!("\nOptions: ");
            println!("    --version | Prints the toolinst version.");
            println!("    --help | Prints this message.");
            
            return Some(ExitCode::from(0));
        }
    }

    Some(ExitCode::from(0))
}