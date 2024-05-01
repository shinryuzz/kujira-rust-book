// static な lifetime を持つ文字列リテラルを引数に取る関数
fn echo(s: &'static str) {
    println!("{}", s);
}

fn main() {
    // 文字列リテラル (&'static str) を指定
    echo("inu");
    echo("neko");

    // String 型のライフタイムは 'static より短いため、エラーがになる
    // let s = String::from("buta");
    // echo(&s); // syntax error: `s` does not live long enough borrowed value does not live long enough

    // let s = "uma".to_string();
    // echo(&s)
}
