/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-04 15:10:55
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-04 15:17:53
 * @FilePath: /match-demo/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
#[derive(Debug)]
enum Usstate {
      Alabama,
      Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Usstate),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state)=>{
            print!("State quarter from {:?}!",state);
            25
        }
    }   
}
fn main() {
   let c = Coin::Quarter(Usstate::Alaska);
   println!("{}",value_in_cents(c));
}
