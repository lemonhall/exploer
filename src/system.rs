use std::path::Path;
use std::process::Command;

/// 使用系统默认程序打开文件
pub fn open_file<P: AsRef<Path>>(path: P) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let path = path.as_ref();
        // 在Windows上使用start命令打开文件
        if !path.exists() {
            return Err(format!("文件不存在: {:?}", path));
        }

        let status = Command::new("cmd")
            .args(&["/C", "start", "", &path.to_string_lossy()])
            .status();

        match status {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("无法打开文件: {}", e)),
        }
    }

    #[cfg(target_os = "macos")]
    {
        // 在macOS上使用open命令
        let status = Command::new("open")
            .arg(path.as_ref())
            .status();

        match status {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("无法打开文件: {}", e)),
        }
    }

    #[cfg(target_os = "linux")]
    {
        // 在Linux上使用xdg-open命令
        let status = Command::new("xdg-open")
            .arg(path.as_ref())
            .status();

        match status {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("无法打开文件: {}", e)),
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("不支持的操作系统".to_string())
    }
}

/// 在文件夹中显示文件（即打开包含该文件的文件夹并选中该文件）
pub fn show_in_folder<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let path = path.as_ref();
    
    if !path.exists() {
        return Err(format!("文件不存在: {:?}", path));
    }

    #[cfg(target_os = "windows")]
    {
        // 在Windows上使用explorer命令
        let status = Command::new("explorer")
            .args(&["/select,", &path.to_string_lossy()])
            .status();

        match status {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("无法在文件夹中显示: {}", e)),
        }
    }

    #[cfg(target_os = "macos")]
    {
        // 在macOS上使用open命令带-R参数
        let status = Command::new("open")
            .args(&["-R", &path.to_string_lossy()])
            .status();

        match status {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("无法在文件夹中显示: {}", e)),
        }
    }

    #[cfg(target_os = "linux")]
    {
        // 在Linux上，我们只能打开包含的文件夹
        if let Some(parent) = path.parent() {
            let status = Command::new("xdg-open")
                .arg(parent)
                .status();

            match status {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("无法在文件夹中显示: {}", e)),
            }
        } else {
            Err("无法获取父文件夹".to_string())
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("不支持的操作系统".to_string())
    }
} 