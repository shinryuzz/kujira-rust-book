use std::time::{SystemTime, UNIX_EPOCH};

static mut SEED: u32 = 0;

// 関数自体を unsafe にすることで、関数内での unsafe ブロックを省略できる
unsafe fn rand_global(start: u32, end: u32) -> u32 {
    if SEED == 0 {
        let epoc = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(); // 1970-01-01 00:00:00 からの経過時間
        SEED = epoc.as_millis() as u32;
    }

    SEED ^= SEED << 13;
    SEED ^= SEED >> 17;
    SEED ^= SEED << 5;
    return SEED % (end - start + 1) + start;
}

fn main() {
    for _ in 0..30 {
        unsafe {
            let v = rand_global(1, 6);
            println!("{}", v);
        }
    }
}
