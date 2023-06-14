use std::env;
use std::fs;
use std::os::unix::prelude::PermissionsExt;

// ユーザーの権限を確認して表示
fn user_permission(mode: &u32) {
    let user_read = mode & 0o400 != 0;          // ユーザーの読み取り権限を確認
    let user_write = mode & 0o200 != 0;         // ユーザーの書き込み権限を確認
    let user_executable = mode & 0o100 != 0;    // ユーザーの実行権限を確認

    println!("User: {}{}{}", read_permission(user_read), write_permission(user_write), execute_permission(user_executable));
}

// グループの権限を確認して表示
fn group_permission(mode: &u32) {
    let group_read = mode & 0o40 != 0;
    let group_write = mode & 0o20 != 0;
    let group_executable = mode & 0o10 != 0;

    println!("Group: {}{}{}", read_permission(group_read), write_permission(group_write), execute_permission(group_executable));
}

// その他の権限を確認して表示
fn other_permission(mode: &u32) {
    let other_read = mode & 0o4 != 0;
    let other_write = mode & 0o2 != 0;
    let other_executable = mode & 0o1 != 0;

    println!("Other: {}{}{}", read_permission(other_read), write_permission(other_write), execute_permission(other_executable));
}

fn read_permission(can_i_read: bool) -> String { 
    if can_i_read {
        return "Read ".to_string();
    } else {
        return "".to_string();
    }
}

fn write_permission(can_i_write: bool) -> String {
    if can_i_write {
        return "Write ".to_string();
    } else {
        return "".to_string();
    }
}

fn execute_permission(can_i_execute: bool) -> String {
    if can_i_execute {
        return "Execute".to_string();
    } else {
        return "".to_string();
    }
}

fn main() {
    // コマンドライン引数の受付
    let args: Vec<String> = env::args().collect();
    // ファイル名を取得
    if args.len() < 2 || args.len() >= 3 {
        println!("Usage: {} file_name", args[0]);
        return;
    }
    
    // ファイル名(第2引数)の値を変数に格納
    let file_path = &args[1];
    
    // ファイル名の存在確認
    match fs::metadata(file_path) {
        Ok(metadata) => {
            let permissions = metadata.permissions();
            let mode = permissions.mode();

            user_permission(&mode);
            group_permission(&mode);
            other_permission(&mode);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
