fn main() {
    let c = C{};
    c.a();
    c.a_a();
    c.b();
}

// trait 类似于接口
// impl B for C  为类型C实现B这个trait
// trait B:A    :   代表中继承   B继承于A
// 代表  如果其他类型要实现B这个trait，也必须实现A这个trait

trait A {
    fn a(&self);

    // a_a 可以重写 也可以不重写
    fn a_a(&self){
        println!("我是trait A中非必须重写的方法a_a");
    }
}

trait B:A {
     fn b(&self); 
}


struct C {}
    


impl B for C {
    fn b(&self) {
        println!("为类型C实现tarit B中的必须重写的方法b");
    }
}


impl A for C {
    fn a(&self){
        println!("为类型C实现tarit A中的必须重写的方法a");
    }
}
