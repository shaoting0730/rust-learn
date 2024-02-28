/*
 * @Author: zhoushaoting 510738319@qq.com
 * @Date: 2024-02-28 09:34:36
 * @LastEditors: zhoushaoting 510738319@qq.com
 * @LastEditTime: 2024-02-28 09:52:38
 * @FilePath: /modules_demo/mylib/src/lib.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// 直接导出一个方法
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// 导出模块
pub mod hello {
    pub fn say_hello(name: &str)  {
        println!("Hello, {}!", name);
    }
    // 嵌套模块
    pub mod world {
        pub fn say_world()  {
            println!("嵌套模块");
        }
        // fn fuck(){
        //     println!("fuck,我是私有的");
        // }
    }
}
