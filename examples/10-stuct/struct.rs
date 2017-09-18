/**
1, 增加一个计算长方形面积的函数 rect_area（尝试使用嵌套的解构方式）。
2, 增加一个函数 square，接受的参数是一个 Point 和一个 f32，并返回一个 Rectangle（长方形）的信息，包括左下角的点，以及长和宽的浮点数值。
**/
#[warn(unused_imports)]
use std::fmt;
// 单元结构体
struct Nil;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        p1: Point {x: x1, y: y1},
        p2: Point {x: x2, y: y2},
    } = rect;

    (x1 - x2) * (y1 - y2)
}

fn square(point: Point, n: f32) -> Rectangle {
    let Point {
        x: x1,
        y: y1,
    } = point;
    // let var = n;
    let point2: Point = Point {
       x: x1 + n,
       y: y1 + n,
    };

    Rectangle {
        p1: point,
        p2: point2
    }
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // 实例化结构体 `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用 `let` 绑定来解构 point
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // 实例化一个单元结构体
    let _nil = Nil;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    println!("{:.2}", rect_area(_rectangle));

    let point4: Point = Point{
        x: 1.3,
        y: 3.6
    };

    println!("new Rectangle is  {:?}", square(point4, 3.0));
}