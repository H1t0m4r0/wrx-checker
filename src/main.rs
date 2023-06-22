use colored::*;
use std::env;
use std::fs;
use std::os::unix::prelude::PermissionsExt;

// check_permissionマクロを定義
macro_rules! check_permission {
    ($type: ident, $mode: expr, $read_bit: expr, $write_bit: expr, $execute_bit: expr) => {
        fn $type(mode: &u32) {
            let can_read = mode & $read_bit != 0;
            let can_write = mode & $write_bit != 0;
            let can_execute = mode & $execute_bit != 0;

            println!(
                "{}:\t{} {} {}",
                stringify!($type),
                permission_color(&can_read, "Read"),
                permission_color(&can_write, "Write"),
                permission_color(&can_execute, "Execute")
            );
        }
    };
}

check_permission!(user, mode, 0o400, 0o200, 0o100);
check_permission!(group, mode, 0o40, 0o20, 0o10);
check_permission!(other, mode, 0o4, 0o2, 0o1);

fn permission_color(has_permission: &bool, permission_name: &str) -> ColoredString {
    if *has_permission {
        return permission_name.green();
    } else {
        return permission_name.red();
    }
}

fn main() {
    // コマンドライン引数の受付
    let args: Vec<String> = env::args().collect();
    // ファイル名を取得
    if args.len() < 2 || args.len() >= 3 {
        println!("Usage:\t{} \"file_name\"", args[0]);
        return;
    }

    // ファイル名(第2引数)の値を変数に格納
    let file_path = &args[1];

    // ファイル名の存在確認
    match fs::metadata(file_path) {
        Ok(metadata) => {
            let permission = metadata.permissions();
            let mode = permission.mode();

            user(&mode);
            group(&mode);
            other(&mode);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
