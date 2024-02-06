/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-07 00:36:35
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-07 01:21:34
 * @FilePath: /rust-panic/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use std::{fs::File, io::ErrorKind};

fn main() {
    println!("Hello, world!");

    // panic!("crash");
    // open_file();
    // open_file1();
    // open_file2();
}

fn open_file() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => println!("file opened: {:?}", file),
        Err(error) => println!("file not found: {:?}", error),
    }

    // let f = File::open("hello.txt").unwrap();


    let f = File::open("hello.txt").expect("出错了");

}

fn open_file1() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
    println!("file opened: {:?}", f);
}

fn open_file2() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }       
    });
}