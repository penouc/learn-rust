const const_bind: i32 = 3;

fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    _immutable_binding = 3;
    // const_bind = 4; 
}