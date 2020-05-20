#[derive(Debug)]

struct NewsArticle{
    author:String,
    content:String,
}
struct Tweet{
    username: String,
    content:String,
}

pub trait Summary{
    fn summarize(&self)->String;
}
impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("Author Name: {} \nContent: {}", self.author,self.content)
    }
}

impl Summary for Tweet{
    fn summarize(&self)->String{
        format!("@{} \nContent: {}", self.username,self.content)
    }
}

fn main() {
    let news1=NewsArticle{
        author: String::from("Hina Khadim"),
        content: String::from("Studying Software Engg. in Ned University."),
    };
    let tweet1=Tweet{
        username:String::from("Hina Khadim"),
        content:String::from("Let's Rise together.")
    };
    println!("NEWS ARTICLE: \n\n{}",news1.summarize());
    println!("\nTWITTER: \n\n{}",tweet1.summarize());
}
