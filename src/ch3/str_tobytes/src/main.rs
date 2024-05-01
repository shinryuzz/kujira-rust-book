fn main() {
    let pr = "neko ni koban";

    for c in pr.bytes() {
        print!("{:2x}", c); // 16進数で表示
    }
    println!("");

    println!("bytes size: {}B", pr.len());
}
