/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-23 09:29:57
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-23 13:52:58
 * @FilePath: /ownership_tips/src/tips/two.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
pub fn run1() {
    // 使用&借用  一个变量可以有多个不可变引用 
    // s2 和 s3 都借用了 s1
    let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s1;
    println!("{},{},{}",s1,s2,s3); 
}

pub fn run2(){
    // 可以使用不可变引用 借用 一个可变变量
    let mut s1 = String::from("hello");
    let s2 = &s1;
    println!("{},{}",s1,s2);
}

pub fn run3(){
    // 使用&mut可变引用  一个值只能有一个可变引用
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    let s3: &mut String = &mut s1;
    println!("{}",s2);
}

pub fn run31(){
    // 注意 有排它性   https://rust-book.junmajinlong.com/ch6/04_understand_mutable_ref.html
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    let s3: &mut String = &mut s1;
    s3.push_str(", world");
    println!("{}",s3);
}



