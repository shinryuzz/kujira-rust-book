fn main() {
    for i in 1..101 {
        // 1 ~ 100 まで繰り返す
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i); // 第一引数に文字列を指定する必要があるため "{}" と書く
        }
    }
}
