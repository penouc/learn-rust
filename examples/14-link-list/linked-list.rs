use List::*;

#[derive(Debug)]
enum List {
    // Cons： 元组结构体，包含一个元素和一个指向下一节点的指针
    Cons(u32, Box<List>),
    // Nil： 末结点，表明链表结束
    Nil,
}

impl List {

    fn new() -> List{
        Nil
    }

    fn prepend(self, elem: u32) -> List{
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            // 不能得到 tail 的所有权，因为 `self` 是借用的；
            // 而是得到一个 tail 引用
            Cons(_, ref tail) => 1+ tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}