use std::process::Command;

fn main() {
    // 根据不同的操作系统调用不同的命令
    if cfg!(target_os = "windows") {
        // 确保已经安装cargo-wix
        if Command::new("cargo").args(&["wix", "init"]).status().is_err() {
            panic!("Failed to initialize cargo-wix");
        }
        if Command::new("cargo").args(&["wix"]).status().is_err() {
            panic!("Failed to create MSI");
        }
    } else if cfg!(target_os = "macos") {
        // 确保已经安装cargo-bundle
        if Command::new("cargo").args(&["bundle", "--release"]).status().is_err() {
            panic!("Failed to create bundle and/or package");
        }
    }
}
