fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    print!("the largest number is {}", largest_i32(&number_list));
    let x = Point { x: 1, y: 2 };
    let y = Point { x: 1.0, y: 2.0 };
    let p1 = Point2 { x: 1, y: 2 };
    let p2 = Point2 { x: 1.0, y: 2.0 };
    let p = p1.mixup(p2);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

trait Runnable {
    fn run(&self);
}

trait Print {
    fn print(&self);
}

impl<T: Runnable> Print for T {
    fn print(&self) {
        self.run();
    }
}