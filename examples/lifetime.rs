use std::fmt::Display;

fn main() {
    //声明周期标注是给借用检查器使用的，不能改变变量真正的生命周期
    //right
    let s1 = String::from("hello");
    {
        let s2 = String::from("hello1");
        println!("longest is {}", longest(s1.as_str(), s2.as_str()));
    }
    //wrong
    let result;
    let s1 = String::from("hello");
    {
        let s2 = String::from("hello2");
        result = longest(s1.as_str(), s2.as_str());
    }
    //println!("longest is {}",result);

    //right
    let result;
    let s1 = String::from("hello");
    {
        let s2: &'static str = "hello2";
        //s2较长时,result会获取s2值的所有权
        result = longest(s1.as_str(), s2);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn wrong() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    //println!("r:{}", r);
}

fn combine<'a, T>(x: &'a str, y: &'a str, display: T) -> &'a str
    where T: Display {
    println!("{}", display);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

