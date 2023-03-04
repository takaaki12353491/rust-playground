// Debugのtraitを実装
// dead_codeで未使用のwarningを削除
#[derive(Debug)]
#[allow(dead_code)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[allow(dead_code)]
impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn area(&self) {
        println!("area is {}", self.width * self.height)
    }
}

pub fn _field() {
    let mut user1 = User {
        name: String::from("name"),
        email: String::from("email@example.com"),
        sign_in_count: 1,
        active: true,
    };
    user1.name = String::from("another@example.com");
    // #をつけるとフォーマットしてくれる
    println!("{:#?}", user1);
}

pub fn _rectangle() {
    let rec1 = Rectangle::create(20, 20);
    println!("{:#?}", rec1);
    println!("{:#?}", rec1.area());
}
