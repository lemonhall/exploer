use std::path::Path;
use std::process::Command;
use std::io;

/// 使用系统默认程序打开文件
pub fn open_file(path: &Path) -> io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        let path_str = path.to_string_lossy().to_string();
        Command::new("cmd")
            .args(["/C", "start", "", &path_str])
            .spawn()
            .map(|_| ())
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .map(|_| ())
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map(|_| ())
    }
}

/// 获取驱动器可用空间和总空间
pub fn get_drive_space(path: &Path) -> (u64, u64) {
    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
        
        let mut free_bytes_available = 0u64;
        let mut total_bytes = 0u64;
        let mut total_free_bytes = 0u64;
        
        let path_str = match path.to_str() {
            Some(s) => s,
            None => return (0, 0),
        };
        
        // 确保路径以反斜杠结尾，例如 "C:\"
        let mut path_with_slash = String::from(path_str);
        if !path_with_slash.ends_with('\\') {
            path_with_slash.push('\\');
        }
        
        // 转换为宽字符字符串
        let wide_path: Vec<u16> = OsString::from(path_with_slash)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        unsafe {
            if GetDiskFreeSpaceExW(
                wide_path.as_ptr(),
                &mut free_bytes_available as *mut u64,
                &mut total_bytes as *mut u64,
                &mut total_free_bytes as *mut u64,
            ) != 0
            {
                return (free_bytes_available, total_bytes);
            }
        }
    }
    
    // 默认返回0,0（非Windows系统或出错时）
    (0, 0)
}

/// 在文件夹中显示文件（即打开包含该文件的文件夹并选中该文件）
#[allow(dead_code)]
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