use tokio;

// Modules & its' namespace
mod utils;
use utils::util_functions::{fetch_news, display_articles, get_key};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // getting api key from file
    let api_key: String = match get_key() {
        Ok(body) => body,
        Err(e) => {
            println!("{}", e);
            return Err(e.into());
        }
    };

    // giving user choosing option
    println!("Enter the category of the news: ");
    const CATEGORY: [&str; 7] = ["business", "entertainment", "general", "health", "science", "sports", "technology"];

    for i in 0..CATEGORY.len() {
        println!(" {} => {} ", i, CATEGORY[i])
    }
    println!(" 7 => all category,");
    println!(" 8 => Exit\n");
    
    println!("Enter your news choice:");
    let mut user_choice = String::new();
    std::io::stdin()
        .read_line(&mut user_choice)
        .expect("Didn't enter the string");
    // parsing string to number
    let choice: usize = user_choice
        .trim()
        .parse()
        .expect("please enter a number next time");

    // control flow, for news fetch and display
    if choice == 8 {
    } else if choice <8 {
        let mut _url = String::new();
        if choice == 7 {
            _url = format!("https://newsapi.org/v2/top-headlines?country=us&apiKey={}", api_key);
        }
        else {
            _url = format!("http://newsapi.org/v2/top-headlines?country=us&category={}&apiKey={}", CATEGORY[choice], api_key);
        }

        let articles = fetch_news(_url).await?;
        display_articles(articles);
    }
    else {
        println!("Invalid Category for the news");
    }

    Ok(())
}