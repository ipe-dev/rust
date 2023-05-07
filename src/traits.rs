trait Fruits {
    fn price(&self) -> u32;
}
struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}
struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

trait Summary {
    // fn summarize(&self) -> String;
    // traitにデフォルトの処理を実装することもできる
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
trait Message {
    fn message(&self)-> String {
       String::from("Message") 
    }
}
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{} by {} ({})", self.headline, self.author, self.location)
    // }
}
impl Message for NewsArticle {
    
}
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {} ", self.username, self.content)
    }
}

pub fn run() {
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("ofcourse, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    let article = NewsArticle {
        headline: String::from("たぬきが逃げた"),
        location: String::from("小田原"),
        author: String::from("taro"),
        content: String::from("動物園のたぬきが逃げました"),
    };
    println!("{}", article.summarize());
    notify(&article);
    notify_another(&article);
    // notify_another(&tweet);
}
fn get_price<T: Fruits>(fruits: T) {
    println!("price is: {}", fruits.price());
}

// impl SummaryはSummaryトレイトが実装されている方なら渡せるという意味
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());    
}
fn notify_another(item: &(impl Summary + Message)) {
    println!("Breaking news! {}", item.summarize());    
    println!("Message! {}", item.message());    
}