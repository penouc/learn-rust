// debug-sto 调试输出

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("{:?} months in a years.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
    "Slater",
    "Christian",
    actor="actor's");

    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));
}