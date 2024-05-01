fn main() {
    let g1 = String::from("hello");
    let g2 = g1;
    // let g2 = g1.clone(); // この場合 g1 は破棄されない
    // println!("{}", g1); // compile 時に `value borrowed here after moved`というエラーが出る
    println!("{}", g2);
}
