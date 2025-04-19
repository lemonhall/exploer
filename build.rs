use std::io::Write;
use std::path::Path;
use std::fs;

fn main() {
    if cfg!(target_os = "windows") {
        // 设置源和目标路径
        let assets_dir = Path::new("src/assets");
        let icon_source_path = assets_dir.join("icon.ico");
        
        // 检查assets目录中是否有图标文件
        if icon_source_path.exists() {
            println!("在assets目录找到图标文件");
            
            // 设置Windows应用程序图标
            let mut res = winres::WindowsResource::new();
            res.set_icon(icon_source_path.to_str().unwrap());
            
            // 尝试编译资源
            match res.compile() {
                Ok(_) => println!("成功设置Windows资源图标"),
                Err(e) => println!("设置Windows资源图标失败: {:?}", e),
            }
        } else {
            // 输出提示信息
            println!("警告: 未找到图标文件 src/assets/icon.ico");
            println!("请使用GIMP或其他工具将SVG图标转换为ICO格式");
            println!("将转换后的图标放在 src/assets/icon.ico 位置");
            
            // 创建一个提示文件
            let mut note = fs::File::create(assets_dir.join("icon_note.txt")).unwrap_or_else(|_| {
                fs::create_dir_all(assets_dir).expect("无法创建assets目录");
                fs::File::create(assets_dir.join("icon_note.txt")).expect("无法创建提示文件")
            });
            
            writeln!(note, "请将以下SVG文件之一转换为ICO格式:").unwrap();
            writeln!(note, "- src/assets/icon.svg").unwrap();
            writeln!(note, "- src/assets/icon2.svg").unwrap();
            writeln!(note, "- src/assets/icon3.svg").unwrap();
            writeln!(note, "转换后将文件命名为icon.ico并放在assets目录下").unwrap();
        }
        
        // 标记需要重新运行的条件
        println!("cargo:rerun-if-changed=src/assets/icon.ico");
        println!("cargo:rerun-if-changed=build.rs");
    }
} 