/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-28 09:31:42
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-28 09:55:55
 * @FilePath: /modules_demo/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
// 根目录执行 cargo new –lib 模块名
// 模块可以多层级嵌套
// 在 根目录中的Cargo.toml 中引入：
//  [dependencies] 
//  mylib = { path = "./mylib" }
use mylib::hello::say_hello;
use mylib::hello::world::say_world;
use mylib::add;

fn main() {
    say_hello("Rust学习");
    say_world();
    let num =  add(20, 30);
    println!("{}", num);
}
