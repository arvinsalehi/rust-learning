mod config;

use config::posts::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("Arvin"),
        content: String::from("Arvin"),
        retweet:true,
    };
    let news_article_1 = NewsArticle {
        author: String::from("The honorable shorty"),
        content: String::from("The world we live in"),
    };

    tweet.summerize();
    news_article_1.summerize();
}
