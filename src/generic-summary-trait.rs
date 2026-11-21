// Challenge: Define a trait Summary with a method fn summarize(&self) -> String.
// Implement it for two structs: Article { title: String, body: String } and Tweet { username: String, content: String }.
// Then write a function fn notify(item: &impl Summary) that prints the summary.

fn main() {
    let article: Article = Article {
        title: "Second judge to block Trump's contested D.C. arch".to_string(),
        body: "A second federal judge will block construction on President Trump's embattled arch set to be erected near Arlington National Cemetery".to_string(),
    };

    let tweet: Tweet = Tweet {
        username: "ms fan club".to_string(),
        content: "Major Microsoft outage".to_string(),
    };

    notify(&article);
    notify(&tweet);
}

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    body: String,
}
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.body)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}