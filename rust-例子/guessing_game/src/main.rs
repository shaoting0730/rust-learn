use std::io;

/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-01-31 13:26:30
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-01-31 15:19:29
 * @FilePath: /guessing_game/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
fn main() {
    println!("猜数!");
    print!("---");
    println!("猜测一个数");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("读取失败");
    println!("你猜测的数是: {}", guess);
}
