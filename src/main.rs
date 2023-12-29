use std::env;
use std::process::{Command, exit};

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查参数是否正确
    if args.len() < 2 {
        eprintln!("Usage: of <directory>...");
        exit(1);
    }

    // 遍历参数列表中的目录
    for directory in args[1..].iter() {
        // 检查操作系统是否为 Windows
        if cfg!(windows) {
            // 使用 `explorer` 命令打开文件夹管理器并定位到当前目录
            let output = Command::new("explorer")
                .arg(directory)
                .output();

            // 检查命令执行结果
            match output {
                Ok(_) => println!("Windows Explorer opened at: {}", directory),
                Err(e) => eprintln!("Failed to open Windows Explorer: {}", e),
            }
        } else {
            // 使用 `open` 命令打开 Finder 并定位到当前目录
            let output = Command::new("open")
                .arg("-a")
                .arg("Finder")
                .arg(directory)
                .output();

            // 检查命令执行结果
            match output {
                Ok(_) => println!("Finder opened at: {}", directory),
                Err(e) => eprintln!("Failed to open Finder: {}", e),
            }
        }
    }
}
