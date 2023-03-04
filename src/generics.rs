// structで構造体を作れる
// ジェネリクス型を指定できる
struct _Point<T, U> {
    x: T,
    y: U,
}

// 構造体の関数を定義
impl<T, U> _Point<T, U> {
    fn _mixup<V, W>(self, other: _Point<V, W>) -> _Point<T, W> {
        _Point {
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
    let p1 = _Point { x: 1, y: "p1" };
    let p2 = _Point { x: 2, y: "p2" };
    let p3 = p1._mixup(p2);
    println!("p3 value is {} and {}", p3.x, p3.y);
}
