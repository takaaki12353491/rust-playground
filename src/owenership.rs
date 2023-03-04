pub fn _copy() {
    let i1 = 1;
    // static memoryを参照する型はコピーできるのでi1が使える
    let i2 = i1;
    println!("i1 value is {}", i1);
    println!("i2 value is {}", i2)
}

pub fn _not_copy() {
    let s1 = String::from("hello");
    // heap memoryを所有する型はコピーできないのでs1が使えない
    let s2 = s1;
    println!("s2 value is {}", s2);
}

pub fn _deep_copy() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // s1とs2は異なるアドレスになる
    println!("s1 stack address value is {:p}", &s1);
    println!("s1 stack address value is {:p}", &s2);
}

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

// 呼び出し元でまた使える
fn _giveback(s: String) -> String {
    s
}

// アドレスを渡すことで処理だけ分割できる
fn _operate(s: &String) -> usize {
    s.len()
}

// mutableだと値を操作できる
fn _change(s: &mut String) {
    s.push_str(" world");
}
