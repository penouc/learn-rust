fn main() {
    fn function (i: i32) -> i32{ i+1 }

    let closure_anotated = |i: i32| -> i32{ i + 1 };
    let closure_inferd = |i| i + 1;

    let i = 1;
    println!("function :{}", function(i));
    println!("anotated: {}", closure_anotated(i));
    println!("infferd: {}", closure_inferd(i));

    let one = || 1;
    println!("closure returning one: {}", one());

}