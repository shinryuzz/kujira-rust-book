fn encrypt(text: &str, shift: i16) -> String {
    // '' ... char (char)
    // "" ... string (&str)
    let code_a = 'A' as i16;
    let code_b = 'Z' as i16;

    let mut result = String::new();

    for ch in text.chars() {
        let mut code = ch as i16;

        if code_a <= code && code <= code_b {
            code = (code - code_a + shift + 26) % 26 + code_a;
        }

        result.push((code as u8) as char);
    }
    return result;
}

fn main() {
    let enc = encrypt("I LOVE YOU.", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec)
}
