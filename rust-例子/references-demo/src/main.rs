fn main() {
   let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize{
    s.len()
}

//----------------------------------------------
// fn main() {
//     let s1 = String::from("hello");
//      let len = calculate_length(&s1);
//      println!("The length of '{}' is {}.", s1, len);
//  }
//  fn calculate_length(s: &String) -> usize{
//     //  s.push_str("world");  // cannot borrow `*s` as mutable, as it is behind a `&` references` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//      s.len()
//  }

//------------------------------------------------
// fn main() {
//     let mut s1 = String::from("hello");
//      let len = calculate_length(&mut s1);
//      println!("The length of '{}' is {}.", s1, len);
//  }
//  fn calculate_length(s: &mut String) -> usize{
//      s.push_str("world");  // cannot borrow `*s` as mutable, as it is behind a `&` references` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//      s.len()
//  }

//-----------------------------------------------
// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;  // cannot borrow `s` as mutable more than once at a time second mutable borrow occurs here
//     println!("{}, {}", r1, r2);
//  }

//  fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s;
//     let r2 = &s;  // ok
//     println!("{}, {}", r1, r2);
//  }

// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     {
//         let r2 = &mut s; // ok
//     }
// }

//-----------------------------------------------
// fn main() {
//     let mut s = String::from("hello");
//     let _r1= &s;
//     let _r2= &s;
//     let _r3: &mut String= &mut s;  //ok
// }

//-----------------------------------------------
// fn main() {
//     let r = dangle();
// }

// // missing lifetime specifier this function's return type contains a borrowed value, but there is no value for it to be borrowed fromrustc
// fn dangle() -> &String { 
//     let s = String::from("hello");
//     &s  // return a reference to the String s, but when the function returns, s will be deallocated
// }