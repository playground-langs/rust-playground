fn main() {
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}", s);

    let s1 = String::from("hello");

    //error
    // let s2 = s1;
    //println!("{}",s1);

    let s2 = s1.clone();

    println!("{}", s1);

    println!("{}", s1);

    takes_ownership(s2);

    //println!("{}",s2);

    let x = 5;

    makes_copy(x);

    println!("{}", x);

    let mut y = String::new();

    borrow(&mut y);

    println!("{}",y);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: u32) {
    println!("{}", x);
}

fn borrow(s: &mut String) {
    s.push_str("test");
}