/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-02 12:56:55
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-21 09:46:05
 * @FilePath: /undefined/Users/zhoushaoting/Desktop/GitHub/rust-learn/rust-例子/move-demo/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
fn main() {
    let s = String::from("hello");
    func_one(s); // 发生了move 传递String的所有权  String是在堆上保存，引用传递
    // println!("{}", s); // borrow of moved value: `s` value borrowed here after move

    let x = 5; // 值传递 i32在栈上
    func_two(x);
    println!("{}", x);

    let y = "str"; // 值传递 str在栈上
    func_three(y);
    println!("{}", y);
}

fn func_one(s: String) {
    println!("{}", s);
}

fn func_two(i: i32) {
    println!("{}", i);
}

fn func_three(i: &str) {
    println!("{}", i);
}
