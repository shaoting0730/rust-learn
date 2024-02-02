/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-02 16:04:02
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-02 16:08:53
 * @FilePath: /slice-demo/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
fn main() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("{}", word);
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
