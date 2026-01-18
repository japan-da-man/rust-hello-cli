use std::env;
use std::process;

use rust_hello_cli::Config;

// 使用例
// cargo run --example main query filename
// query: 検索キーワード ex. poe
// filename: 検索対象ファイル ex. poem.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}", err);
        process::exit(1)
    });

    if let Err(err) = rust_hello_cli::run(config) {
        println!(" Apprication error: {}", err);

        process::exit(1);
    };
}
