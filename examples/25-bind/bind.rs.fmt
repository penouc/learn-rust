
fn age() -> u32 {
    15
}

fn main() {
    println!("Tell me type of person you are");

    match age() {
        0 => println!("I'm not born I guess"),
        n @ 1  ... 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ... 19 => println!("I'm a teen of age {:?}", n),
        // 没有绑定。返回结果。
        n             => println!("I'm an old person of age {:?}", n),
    }
}


