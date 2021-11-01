fn main() {
    let mut x = 5;
    println!("x={}", x);
    x = 6;
    println!("x={}", x);
    let x = "shadow";
    println!("x={}", x);

    println!("0o11={}", 0o11);

    let tup = (1.0, 1, 200);
    let (x, y, z) = tup;

    println!("x={},y={},z={}", x, y, z);

    println!("x={},y={},z={}", tup.0, tup.1, tup.2);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for x in a {
        println!("{}",x);
    }

    let b = [10; 5];
}