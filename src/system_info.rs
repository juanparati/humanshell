use std::{
    env,
    path::Path,
};
use sysinfo::System;

pub struct SystemInfo {
    pub os: String,
    pub os_version: String,
    pub shell: String,
    pub is_root: bool,
}

impl SystemInfo {
    pub fn new() -> Self {
        System::new_all().refresh_all();

        // Get shell path from the environment variable
        let shell_path = if cfg!(unix) {
            env::var("SHELL").unwrap_or_else(|_| String::from("/bin/sh"))
        } else if cfg!(windows) {
            env::var("COMSPEC").unwrap_or_else(|_| String::from("cmd.exe"))
        } else {
            String::from("unknown")
        };

        let path = Path::new(&shell_path);

        SystemInfo {
            os: System::name().unwrap_or_default(),
            os_version: System::os_version().unwrap_or_default(),
            shell: path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| "unknown".to_string()),
            is_root: env::var("USER").unwrap_or_default() == "root",
        }
    }
}
