fn main() {
    for j in 1..10 {
        let s = (1..10)
            .map(|i| format!("{:3}", i * j))
            .collect::<Vec<String>>()
            .join(",");
        println!("{}", s);
    }
}
