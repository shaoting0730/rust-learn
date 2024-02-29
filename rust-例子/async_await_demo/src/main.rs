/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-28 14:46:13
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-28 16:11:24
 * @FilePath: /async_await_demo/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use std::{thread::sleep, time::Duration};
use async_std::task::{self, spawn};

fn main() {
    println!("Hello, world!");
    // 1
    // do1();
    // do2();

    // 2
    //  let do1_spawn = std::thread::spawn(do1);
    //  let do2_spawn = std::thread::spawn(do2);
    //  do1_spawn.join().unwrap();
    //  do2_spawn.join().unwrap();

    // 3
    // task::block_on(async {
    //     do22();
    // })

}

fn do1(){
    for i in 1..=5 {
        println!("do1 {}",i);
        sleep(Duration::from_millis(500));
    }
}

fn do2(){
    for i in 1..=5 {
        println!("do2 {}",i);
        sleep(Duration::from_millis(1000));
    }
}

async fn do11(){
    for i in 1..=5 {
        println!("do11 {}",i);
        sleep(Duration::from_millis(500));
    }
}

async fn do22(){
    for i in 1..=5 {
        println!("do22 {}",i);
        sleep(Duration::from_millis(500));
    }
}

