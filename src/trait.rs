trait Fruit {
    fn price(&self) -> u32;
}

struct Apple;
impl Fruit for Apple {
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;
impl Fruit for Banana {
    fn price(&self) -> u32 {
        10
    }
}

trait Summary {
    // 実装するとデフォルト処理になる
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
trait Message {
    fn message(&self) -> String {
        String::from("Message")
    }
}

#[allow(dead_code)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
}
impl Summary for NewsArticle {}
impl Message for NewsArticle {}

struct Tweet {
    name: String,
    content: String,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {} ", self.name, self.content)
    }
}

#[allow(dead_code)]
fn get_price<T: Fruit>(fruit: T) {
    println!("price is {}", fruit.price());
}

#[allow(dead_code)]
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// +でSummaryとMessage両方実装しているものを受け取る
#[allow(dead_code)]
fn notify_another(item: &(impl Summary + Message)) {
    println!("Breaking news! {}", item.summarize());
    println!("Message! {}", item.message());
}

pub fn _run() {
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);

    let tweet = Tweet {
        name: String::from("name"),
        content: String::from("content"),
    };
    println!("new tweet {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("hoge"),
        location: String::from("fuga"),
        author: String::from("piyo"),
    };
    println!("new article {}", article.summarize());
    notify(&article);
    // tweetはMessageを実装していないので渡せない
    notify_another(&article);
}
