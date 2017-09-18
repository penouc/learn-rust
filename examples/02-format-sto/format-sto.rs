// 格式化输出

fn main() {
    
    println!("{} days!", 31);

    println!("{0}, this is {1}; {1}, this is {0}", "Alice", "Bob");

    println!("{object}{verb}{subject}", object="I", verb=" ate", subject=" chicken");

    println!("{} of {:b} people know binary;", 1, 2);

    println!("{number:>width$}",number=1,width=6);

    println!("{number:>0width$}",number=1,width=6);

    println!("{number:<0width$}",number=1,width=6);

    #[allow(dead_code)]
    struct Structure(i32);

}