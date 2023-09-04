use std::path::PathBuf;

pub struct ToolInst {
    pub install_path: PathBuf,
    pub compressed_dir: PathBuf,
    pub installer_dir: PathBuf
}

/// Parses an `str` in the configuration file format for toolinst and returns 
/// defined configs.
pub fn parse_config(configs: &str) -> ToolInst {
    let mut install_path = String::new();
    let mut compressed_dir_name = String::from(".compressed");
    let mut installer_dir_name = String::from(".installer");

    for (i, line) in configs.split('\n').enumerate() {
        let config: Vec<&str> = line.split('=').collect();
        
        match config[0].trim() {
            "INSTALL" => install_path = String::from(config[1].trim()),
            "COMPRESSED_DIR_NAME" => compressed_dir_name = String::from(config[1].trim()),
            "INSTALLER_DIR_NAME" => installer_dir_name = String::from(config[1].trim()),
            "" => {},
            _ => eprintln!("Warning! While reading config file, unknow config at line {}: \"{}\"", i + 1, config[0].trim())
        }
    }

    let install_path = PathBuf::from(install_path);
    let compressed_dir = install_path.join(compressed_dir_name);
    let installer_dir = install_path.join(installer_dir_name);

    if !install_path.is_dir() {
        eprintln!("Warning! The defined install path is not a directory or it doesn't exist!");
    }

    ToolInst { install_path, compressed_dir, installer_dir }
}
