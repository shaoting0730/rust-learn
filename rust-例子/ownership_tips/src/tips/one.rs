/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-22 16:42:12
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-23 11:48:38
 * @FilePath: /ownership_tips/src/tips/one.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
// pub struct One {}

// impl One {
//     pub fn run(){
//       println!("one");
//     }
// }

pub fn run1() {
    // s1为字符串,自身并未实现copy trait，s2=s1:所有权从s1到了s2,s1变为未初始化状态,而rust不允许使用未初始化的值.
    let s1 = String::from("hello");
    let s2 = s1;
    //  println!("{}", s1);  // 报错
}

pub fn run2() {
    // 所有权从 s1 到 s2 再到 s3
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2;
    //  println!("{}", s2);  // 报错
}

pub fn run3() {
    // rust中标量类型默认都实现cooy trait,
    // 标量类型包括整数、浮点数、字符和布尔值等简单的数据类型
    let i1 = 10;
    let i2 = i1;
    let i3 = i2;
    let i4 = i1;
    println!("{},{},{},{}", i1, i2, i3, i4);
}

pub fn run4() {
    let s1 = "hello";
    let s2 = s1;
    let s3 = s1;
    println!("{},{},{}", s1, s2, s3);
}
