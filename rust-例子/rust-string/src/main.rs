fn main() {
  crate_string();
  string_len();
}

fn crate_string() {
    let s1 = "hello".to_string();
    let mut s2 = String::from("world");

    s2.push_str("你好");
    s2.push('啊');

    let _s3 = s2 + &s1;

    let s4 = "你好呀".to_string();
    format!("{}-{}", s1, s4);

}

fn string_len() {
    let s = "家hi大幅度".to_string();
    println!("{}",s.len());
    println!("{}",s.chars().count());
    println!("{}",s.bytes().count());
    // &s[0]  error rust不允许下标获取 因为 btf-8 可能获取不到需要的
    println!("{}", &s[0..3]); // 家 有可能获取到非法
    println!("{}", &s[0..1]); //  有可能获取到非法 byte index 1 is not a char boundary; it is inside '家' (bytes 0..3) of `家hi大幅度`

}