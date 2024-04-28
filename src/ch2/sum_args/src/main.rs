fn main() {
    let args = std::env::args();
    let mut total = 0.0;
    for (i, s) in args.enumerate() {
        if i == 0 {
            continue;
        }
        let num = s.parse().unwrap_or(0.0);
        total += num
    }

    println!("Total: {}", total)
}
