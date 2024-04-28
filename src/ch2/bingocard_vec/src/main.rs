use rand::seq::SliceRandom;

fn main() {
    let mut nums = vec![];
    for i in 1..=75 {
        nums.push(i);
    }

    // shuffle
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng); // rng が mutable であることを明示するために &mut rng としている

    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x; // immutable (for の内部では異なるスコープであるため毎回 初期化しても良い )
            if i == 12 {
                print!(" *,"); // nums には i32 しか入らないため、* 母いらない。
            } else {
                print!("{:3}, ", nums[i]);
            }
        }
        println!("");
    }
}
