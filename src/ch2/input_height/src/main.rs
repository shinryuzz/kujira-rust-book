fn main() {
    let mut height;

    loop {
        // while true は非推奨であるため loopを使う
        println!("How tall is your height (cm) ?");
        let def = 0.0; // default value
        height = input_f(def);
        if height > def {
            break;
        }
        println!("Input correct values")
    }

    let weight = 22.0 * (height / 100.0).powf(2.0);
    println!("Your standard weight is {:.1}kg.", weight)
}

fn input_str() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("error"); //
    s.trim_end().to_string()
}

fn input_f(def: f64) -> f64 {
    let s = input_str();

    // unwrap_or() はエラーが発生した場合にデフォルト値を返す
    s.trim().parse().unwrap_or(def)

    // match s.trim().parse() {
    //     Ok(v) => v,
    //     Err(_) => def,
    // }
}
