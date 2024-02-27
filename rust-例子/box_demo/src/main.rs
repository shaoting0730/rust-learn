/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-27 16:40:52
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-27 16:46:25
 * @FilePath: /box_demo/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
fn main() {
    // 把 栈上的数据 搬到 堆上的能力，叫做装箱。
    // Box ：可以把数据存在堆上，而不是栈上。
    let a = 1;  // 存放在栈上
    let b = Box::new(1);  // 存放在堆上
    
    println!("a = {}, b = {}", a, *b);
    // 注意：*b 表示解引用，取出堆上的值
}
