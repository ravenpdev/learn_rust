// Traits
//
//
// Note: It isn't possible to call the default implementation from an overriding implementation of
// that same method.

mod aggregator {
    use std::fmt::Display;

    pub struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }

    impl NewsArticle {
        pub fn new() -> Self {
            NewsArticle {
                headline: String::from(""),
                location: String::from(""),
                author: String::from("newauthor"),
                content: String::from(""),
            }
        }
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            String::from("NewsArticle summarize called.")
        }

        fn summarize_author(&self) -> &str {
            &self.author[..]
        }
    }

    impl Display for NewsArticle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    pub struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    impl Tweet {
        pub fn new() -> Self {
            Tweet {
                username: String::from("tweetuser"),
                content: String::from(""),
                reply: false,
                retweet: false,
            }
        }
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            String::from("Tweet summarize called.")
        }

        fn summarize_author(&self) -> &str {
            &self.username
        }
    }

    pub struct FakeNews {
        headline: String,
        content: String,
        author: String,
    }

    impl FakeNews {
        pub fn new() -> Self {
            FakeNews {
                headline: String::from("fakenews"),
                content: String::from("Fake news content"),
                author: String::from("fakeuser"),
            }
        }
    }

    impl Summary for FakeNews {
        fn summarize_author(&self) -> &str {
            &self.author
        }
    }

    // Defining a Trait
    pub trait Summary {
        fn summarize_author(&self) -> &str;

        fn summarize(&self) -> String {
            format!("Read more from {}...", self.summarize_author())
        }
    }
}

use std::fmt::{Debug, Display};

use aggregator::{FakeNews, NewsArticle, Summary, Tweet};

// Trait as Parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
//
// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer
// form known as a trait bound; it looks like this:
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
//
// We can also specify more than one trait bound. Say we wanted notify to use display formatting as
// well as summarize on item: we specify in the notify definition that item must implement both
// Display and Summary. We can do so using  + syntax:
pub fn notify3<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses
//
//
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    32
}

// Returning Types That Implement Traits
//
// However, you can only use impl Trait if you're returning a single type.
//
// The following snippets will not work:
//
// Returning either a NewsArticle or Tweet isn't allowed due to restrictions around how the impl
// Trait syntax is implemented in the compiler.
//
// fn return_summarizable(switch: bool) -> impl Summary {
//      if switch {
//          NewsArticle::new()
//      } else {
//          Tweet::new()
//      }
// }
fn returns_summarizable() -> impl Summary {
    Tweet::new()
}

// Using Trait Bounds to Conditionally Implements Methods
struct Pair<T> {
    x: T,
    y: T,
}

// Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd
// trait that enables comparison and Display trait that enables printing.
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x ={}", self.x);
        } else {
            println!("The largest member is y ={}", self.y);
        }
    }
}

// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         let mut s = String::new();
//         std::fmt::write(&mut s, format_args!("{}", self)).unwrap();
//         s
//     }
// }

fn main() {
    let news_article = NewsArticle::new();
    let tweet = Tweet::new();
    let fake_news = FakeNews::new();

    println!("{}", news_article.summarize());
    println!("{}", tweet.summarize());
    println!("{}", fake_news.summarize());

    notify(&news_article);
    notify(&tweet);
    notify(&fake_news);

    let s = 3.to_string();
    println!("{}", s);

    // ToString
}
