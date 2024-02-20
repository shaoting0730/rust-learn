/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-20 15:37:37
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-20 15:48:42
 * @FilePath: /dereference_demo/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
fn main() {
    println!("Hello, world!");
    let a = 100;
    add_num(a);
    println!("外面是{}",a);


    let mut a = 100;
    add_num1(&mut a);
    println!("~外面是{}",a);


}

fn add_num(mut a: i32) {
    a = a + 10;
    println!("里面是{}", a)
}

fn add_num1(a:&mut i32){
    *a = *a + 20;
    println!("~里面是{}",a);
}
