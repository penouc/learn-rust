fn main() {
    
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    } 

    let not_a_reference = 3;

    let ref is_a_reference = 4;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
         // 获得一个引用。在增加内容之前，要先得到解引用（Gotta
         // dereference it before we can add anything to it）。
         *m += 10;
         println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}