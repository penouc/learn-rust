fn main() {
    
    let number = 19;

    println!("Tell me the Number {}", number);

    match number{
        1 => println!("It's one"),
        2 | 3 | 4 | 5 | 7 =>  println!("print others number"),
        9...13 => println!("print other others"),
        _ => println!("elses"),
    }

    let boolean = true;
    let binary = match boolean{
        true => 1,
        false => 0,
    };

    println!("{} -> {}", boolean, binary);
}