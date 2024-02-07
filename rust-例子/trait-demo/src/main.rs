/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-07 21:08:36
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-07 23:29:29
 * @FilePath: /trait-demo/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

use trait_demo::Summary;
use trait_demo::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("学习"),
        content: String::from("rust的trait"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let str_list = vec![String::from("学习"), String::from("rust")];
    let result = largest(&str_list);
    println!("The largest string is {}", result);

}   

fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest     
}
