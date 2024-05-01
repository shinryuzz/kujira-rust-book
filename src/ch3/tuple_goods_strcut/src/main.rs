struct Item(String, i64);

fn main() {
    let banana = Item("banana".to_string(), 100);
    let apple = Item("apple".to_string(), 150);
    let mango = Item("mango".to_string(), 300);

    let items = vec![banana, apple, mango];

    let total = calc_total_price(&items);
    println!("Total price is {}.", total);
}

fn print_tuple(item: &Item) {
    println!("Name: {}, Price: {}", item.0, item.1);
}

fn calc_total_price(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    for item in items {
        // for 文で item を取り出す時に item を参照する形になっている
        // だから &item とすると、&&Item 型を渡すことになる
        // しかし エラーには起きない。
        print_tuple(&item);
        total += item.1;
    }
    return total;
}
