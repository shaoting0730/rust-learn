/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-07 21:29:30
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-07 22:57:00
 * @FilePath: /trait-demo/src/泛型.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
//----------------------------------------------------
//  struct Point<T> {
//     x:T,
//     y:T,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//     let float = Point { x: 1.0, y: 4 };  // error  mismatched types expected floating-point number, found integer

// }

//----------------------------------------------------
// struct Point<T,U> {
//     x:T,
//     y:U,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//     let float = Point { x: 1.0, y: 4 };  

// }

//--------------------------------------------------------
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}