use std::{fs, io, thread, time};
use std::result::Result::Ok;

fn main() {
    let mut old_log = String::new();
    let mut no_change_du = 0;

    println!("请选择监控的文件目录，1=奥秘法，2=偶数萨满");
    let path1 =
        "C:\\Users\\xiaotian\\炉石研究\\奥秘法\\Routines\\DefaultRoutine\\Silverfish\\UltimateLogs";
    let path2 =
        "C:\\Users\\xiaotian\\炉石研究\\偶数萨\\Routines\\DefaultRoutine\\Silverfish\\UltimateLogs";

    let path;
    let mut choose = String::new();
    io::stdin()
        .read_line(&mut choose)
        .expect("Failed to read line");
    match choose.as_str().trim() {
        "1" => path = path1,
        "2" => path = path2,
        _ => {
            println!("输入不合法");
            return;
        }
    }

    if let Some(last_log) = get_last_log(path) {
        old_log = last_log;
    }

    loop {
        if let Some(last_log) = get_last_log(path) {
            if old_log.eq(&last_log) {
                print!("{:4} 秒后 文件没有改变  ", no_change_du);
                no_change_du += 10;
            } else {
                old_log = last_log;
                no_change_du = 0;
                println!(" ----------------->>>>>>>>>>>>>>");
                println!(" ----------------->>>>>>>>>>>>>>");
            }
        }
        println!("当前最新的文件是： {:#?}", old_log);

        thread::sleep(time::Duration::from_secs(10));
        if ((no_change_du % 1200) < 35) && (no_change_du > 1200) {
            send_msg();
        }
    }
}

pub fn get_last_log(path: &str) -> Option<String> {
    let entries = fs::read_dir(path);
    if let Ok(res) = entries {
        if let Some(s1) = res.last() {
            if let Ok(res2) = s1 {
                if let Ok(res3) = res2.file_name().into_string() {
                    return Some(res3);
                }
            }
        }
    }
    None
}

pub fn send_msg() {
    let _resp = reqwest::blocking::get(
        "https://api.telegram.org/bot1197004282:AA\
        HXmnpk3scQYrafmAwbv-P9WBk0KL87QjM/sendMessa\
        ge?chat_id=628620014&text=\"爷爷，日志文件好久没变啦，快看看吧！\"",
    );
}

#[test]
fn say_new() {}
