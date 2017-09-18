#![allow(unreachable_code)]

fn main() {
    let mut count_o = 0u32;

    'outer: loop {
        count_o += 1;
        println!("Entered the outer loop");
        let mut count_i = 0u32;
        
        'inner: loop {
            count_i += 1;
            println!("Entered the inner loop");
            // 这只是中断内部的循环
            if count_i == 5 {
                print!("break at count_i = {}", count_i);
                break;
            }

            // 这会中断外层循环
            if count_o == 3 {
                print!("break at count_o = {}", count_o);
                
                break 'outer;     
            }
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}