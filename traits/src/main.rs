pub struct NewsArticles {
    author: String,
    content: String,
    headline: String,
}

pub struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

fn main() {
    let tweet = Tweet {
        username: String::from("name"),
        content: String::from("content"),
        reply: true,
        retweet: false,
    };

    print!("{}", tweet.summarize())
}
