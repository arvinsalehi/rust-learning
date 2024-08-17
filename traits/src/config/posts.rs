pub trait Summary {
    fn summerize(&self) -> String;
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
}

pub struct NewsArticle {
    pub author: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summerize(&self) -> String {
        format!("{} written by {}", self.content, self.username)
    }
}

impl Summary for NewsArticle {
    fn summerize(&self) -> String {
        format!("{} written by {}", self.content, self.author)
    }
}