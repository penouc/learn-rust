fn main() {
    let pair = (2, -2);

    match pair{
        (x, y) if x == y => println!(" x == y"),
        (x, y) if x+y == 0 => println!("antimatter, kaboom!"),
        (x, _) if x%2 == 1 => println!("the first is odd"),
        _ => println!("no correlation..."),
    }
}