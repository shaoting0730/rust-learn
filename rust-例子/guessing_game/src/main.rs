use std::io;
use rand::Rng;
/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-01-31 13:26:30
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-01-31 23:18:25
 * @FilePath: /guessing_game/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
fn main() {
    println!("猜数!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    print!("---");
    println!("猜测一个数");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败");
    
        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数是: {}", guess);
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("太小了!"),
            std::cmp::Ordering::Greater => println!("太大了!"),
            std::cmp::Ordering::Equal => {
                println!("对了!");
                break;
            },
        }
    }
}
