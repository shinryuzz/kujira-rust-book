fn main() {
    let s = "所有権の問題が生じないようになっている".to_string();
    echo(s); // <- ここで所有権が移動してしまう
    println!("{}", s); // だからここでエラーが起きる
}

// echo の引数s は所有権を必要とする型である
fn echo(s: String) {
    println!("{}", s);
}
