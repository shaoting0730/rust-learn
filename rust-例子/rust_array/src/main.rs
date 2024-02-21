use std::fmt::Debug;
fn main() {
    let arr = [1, 2, 3, 4, 5]; // 数组是栈中分配的
    show_array(arr); // 这里需要使用as_mut_slice方法来获取可变引用
    println!("再次输出数组{:?}", arr);
    println!("----------------------------------------------------");  


    let arr: [&str;5] = ["z","x","c","v","b"];
    change_array(arr); // 参数是 值传递
    println!("再次输出数组 {:?}", arr);  
    println!("----------------------------------------------------");  

    let mut arr: [&str; 5] = ["","","","",""];
    change_array1(&mut arr); // 参数是 & 引用传递
    println!("外部再输出{:?}",arr);
}

fn show_array<T:Debug>(arr: T) {
    println!("内部输出数组{:?}", arr);
}

fn change_array(mut arr:[&str;5]) {
    arr[0] = "22222";  
    println!("内部修改元素再输出{:?}", arr); 
}

fn change_array1(arr: &mut[&str;5]) {
    arr[0] = "22222";  
    println!("内部修改元素再输出{:?}", arr); 
}




