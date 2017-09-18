#![allow(dead_code)]

enum Status{
    Rich,
    Poor
}

enum Work{
    Civilian,
    Soldier
}

fn main() {
    
    use Status::{Rich, Poor};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status{
        Rich => println!("The rich have lots of money"),
        Poor => println!("The poor have no money......"),
    }

    match work{
        Civilian => println!("Civilians works"),
        Soldier => println!("Soldier fight!"),
    }
}