/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-03 16:49:34
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-03 17:03:02
 * @FilePath: /struct-impl/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect.area());
    println!(
        "{}",
        rect.can_hold(&Rectangle {
            width: 20,
            height: 40
        })
    );
    let s = Rectangle::square(3);
    println!("{:?}", s);
}
