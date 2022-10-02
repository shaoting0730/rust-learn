use rand::Rng; 
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("猜数字的游戏");

    let secret_number = rand::thread_rng().gen_range(1,101); 

    loop{
        println!("请输入你的数子");

        let mut guess = String::new();
    
        io::stdin()
             .read_line(&mut guess)
             .expect("读取行错误");

         let guess: u32 = match guess.trim().parse(){
              Ok(num) => num,
              Err(_) => continue,
         };    
    
        println!("你输入的数是：{guess}");   

        match  guess.cmp(&secret_number) {
            Ordering::Less => println!("太小"),
            Ordering::Greater=>println!("太大"),
            Ordering::Equal =>{
                  println!("你猜对了");
                  break;
            }
        }
    }
  
}
