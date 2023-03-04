pub fn _refference() {
    let mut s1 = String::from("hello");
    let _r1 = &s1;
    let r2 = &mut s1;
    // mutableとimmutableな参照は同時に使えない
    // println!("r1 is {}", _r1);

    // mutableな参照を後で行なっていると所有権を持っていてもエラーになる
    // println!("s1 is {}", s1);
    println!("r2 is {}", r2);
    // mutableな参照が終わったとだと使用できる
    println!("s1 is {}", s1);
}

pub fn _rewrite() {
    let mut s1 = String::from("hello");
    let r1 = &s1;
    let r2 = &s1;
    println!("r1 is {}. r2 is {}", r1, r2);
    let r3 = &mut s1;
    *r3 = String::from("hello world");
}

pub fn _box_pointer() {
    let t1 = (10, String::from("hello"));
    // Stackに保存していた情報をheapに保存する
    let mut b1 = Box::new(t1);
    (*b1).1 += " world";
}
