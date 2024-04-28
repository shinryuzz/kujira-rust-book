use std::fs::File;
use std::io::{BufRead, BufReader}; // std:io から BufRead と BufReader を使う

fn main() {
    let dicfile = "ejdict-hand-utf8.txt";

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("[USAGE] jisho word");
        return;
    } else {
        // rust では 自動的に 参照解除(デリファレンス)する
        println!("{}", &args[1]);
        println!("{}", args[1]);
    }

    let word = &args[1];

    let fp = File::open(dicfile).unwrap();
    let reader = BufReader::new(fp);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.find(word) == None {
            continue;
        }
        println!("{}", line)
    }
}
