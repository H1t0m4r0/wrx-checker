use colored::*;
use filemagic::magic;
use std::env;
use std::fs;
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "file_permissions")]
struct Opt {
    // ファイル名
    #[structopt(parse(from_os_str))]
    file_path: PathBuf,

    // ユーザー権限
    #[structopt(short = "u", long)]
    user: bool,

    // グループ権限
    #[structopt(short = "g", long)]
    group: bool,

    // その他の権限
    #[structopt(short = "o", long)]
    other: bool,
}

// check_permissionマクロを定義
macro_rules! check_permission {
    ($type: ident, $mode: expr, $read_bit: expr, $write_bit: expr, $execute_bit: expr) => {
        fn $type(mode: &u32) {
            let can_read = mode & $read_bit != 0;
            let can_write = mode & $write_bit != 0;
            let can_execute = mode & $execute_bit != 0;

            println!(
                "{}:\t\t{} {} {}",
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

fn print_file_type(opt: &Opt) {
    let file_path = &opt.file_path;
    let magic = magic!().expect("error");

    println!("File type:\t{}", magic.file(&file_path).expect("error"));
}

fn main() {
    // コマンドライン引数の受付
    let args: Vec<String> = env::args().collect();
    // ファイル名を取得
    if args.len() < 2 || args.len() >= 6 {
        println!("Usage:\t{} [FILE NAME] [OPTION]", args[0]);
        return;
    }

    let opt = Opt::from_args();
    // ファイル名(第2引数)の値を変数に格納
    // let file_path = &args[1];
    // ファイル名の存在確認
    match fs::metadata(&opt.file_path) {
        Ok(metadata) => {
            print_file_type(&opt);
            let permission = metadata.permissions();
            let mode = permission.mode();

            // オプション指定時は指定された権限のみを表示
            if opt.user {
                user(&mode);
            }
            if opt.group {
                group(&mode);
            }
            if opt.other {
                other(&mode);
            }

            // オプション無指定時は全ての権限を表示
            if !opt.user && !opt.group && !opt.other {
                user(&mode);
                group(&mode);
                other(&mode);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
