//! `MiniCRM` 构建脚本
//!
//! 负责编译Slint UI文件和处理其他构建时任务。

use std::fs;
use std::path::Path;
use std::process;

fn main() {
    // 编译Slint UI文件
    if let Err(e) = slint_build::compile("ui/main_window.slint") {
        eprintln!("编译Slint UI文件失败: {e}");
        process::exit(1);
    }

    // 为Slint生成的代码添加clippy允许属性
    // 注意：这需要在slint_build::compile之后执行
    add_clippy_allows_to_generated_code();

    // 监听UI文件变化，确保在UI文件修改时重新构建
    println!("cargo:rerun-if-changed=ui/");

    // 确保UI目录存在
    let ui_dir = Path::new("ui");
    if !ui_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(ui_dir) {
            eprintln!("无法创建UI目录: {e}");
            process::exit(1);
        }
    }

    // 输出构建信息
    println!(
        "cargo:rustc-env=BUILD_TIME={}",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!(
        "cargo:rustc-env=BUILD_VERSION={}",
        env!("CARGO_PKG_VERSION")
    );
}

/// 为Slint生成的代码添加clippy允许属性
///
/// 这个函数会在生成的代码文件开头添加必要的allow属性，
/// 以避免clippy对生成代码的严格检查产生错误。
///
/// 由于Slint生成的代码包含大量的unwrap()、panic!()等调用，
/// 这些在框架内部是安全的，但会触发我们的严格clippy规则。
fn add_clippy_allows_to_generated_code() {
    let Ok(out_dir) = std::env::var("OUT_DIR") else {
        eprintln!("警告: OUT_DIR环境变量未设置");
        return;
    };
    let generated_file = Path::new(&out_dir).join("main_window.rs");

    // 等待文件生成完成
    let mut attempts = 0;
    while !generated_file.exists() && attempts < 10 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        attempts += 1;
    }

    if generated_file.exists() {
        match fs::read_to_string(&generated_file) {
            Ok(content) => {
                // 检查是否已经添加了allow属性
                if !content.contains("#[allow(clippy::unwrap_used)]") {
                    let allow_attributes = "// Slint生成的代码 - 允许特定的clippy规则
// 这些规则对于UI框架生成的代码是必要的，框架确保了这些调用的安全性
#[allow(clippy::unwrap_used)]
#[allow(clippy::expect_used)]
#[allow(clippy::panic)]
#[allow(clippy::todo)]
#[allow(clippy::missing_docs_in_private_items)]
#[allow(missing_docs)]
#[allow(missing_debug_implementations)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::cognitive_complexity)]

";

                    let new_content = format!("{allow_attributes}{content}");

                    if let Err(e) = fs::write(&generated_file, new_content) {
                        eprintln!("警告: 无法为生成的代码添加clippy允许属性: {e}");
                    } else {
                        println!("cargo:warning=已为Slint生成的代码添加clippy允许属性");
                    }
                }
            }
            Err(e) => {
                eprintln!("警告: 无法读取生成的代码文件: {e}");
            }
        }
    } else {
        eprintln!("警告: 未找到Slint生成的代码文件");
    }
}
