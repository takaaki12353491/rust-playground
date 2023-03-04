// structで構造体を作れる
// ジェネリクス型を指定できる
#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

// 構造体の関数を定義
impl<T, U> Point<T, U> {
    #[allow(dead_code)]
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Tはジェネリクス型
// Tにトレイト境界を指定できる
pub fn _largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if largest < item {
            largest = item;
        }
    }
    largest
}

pub fn _mixup() {
    let p1 = Point { x: 1, y: "p1" };
    let p2 = Point { x: 2, y: "p2" };
    let p3 = p1.mixup(p2);
    println!("p3 value is {} and {}", p3.x, p3.y);
}
