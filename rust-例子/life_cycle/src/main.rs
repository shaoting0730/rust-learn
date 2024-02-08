/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-08 20:20:53
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-08 20:26:46
 * @FilePath: /life_cycle/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */


fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);   
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y
    }
}