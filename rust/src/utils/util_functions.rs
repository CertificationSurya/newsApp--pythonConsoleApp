use reqwest;
use reqwest::header::{HeaderMap, USER_AGENT};
use serde::{Deserialize, Serialize};

// utils funcs
pub async fn fetch_news(url: String) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "NewsApi/6.9".parse().unwrap());

    let client = reqwest::Client::new();
    let res: reqwest::Response  = client.get(&url).headers(headers).send().await?;
    let mut articles: Vec<Article> = vec![];

    if res.status().is_success() {
        // seeing output
        // let body_text = res.text().await?;
        // println!("News: {}", body_text);

        // assigning jsoned data
        let articles_res: ResponseArticle = res.json().await?;
        articles = articles_res.articles;

    } else {
        println!("News Api Error: {}", res.status());
    }

    Ok(articles)
}

// get Api Key
pub fn get_key() -> Result<String, Box<dyn std::error::Error>> {
    let path = std::path::Path::new("MyApiForNewsApi.key");

    // We could just simply read which will automatically open file
    let api_key: String = match std::fs::read_to_string(path) {
        Err(why) => panic!("Couldn't read file: {} due to {}", path.display(), why),
        Ok(key) => key,
    };
    Ok(api_key)
}

// displayer
pub fn display_articles(articles: Vec<Article>){
    // println!("Articles: {:?}", articles);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear screen

    for (i, article) in articles.iter().enumerate() {
        // & to reference value, not create
        let source_name = &article.source.name; 
        // as_deref() will de-reference our value. In our case string to str.
        let author = article.author.as_deref().unwrap_or("Unknown author");
        let title = &article.title;
        let description = article.description.as_deref().unwrap_or("No description available");

        println!("Article No: {}", i + 1);
        println!("Source: {}", source_name);
        println!("Author: {}", author);
        println!("Title: {}", title);
        println!("Description: {} \n", description);
    }

}


// Types/Struct

// type for newsApi response
#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub id: Option<String>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub source: Source,
    pub author: Option<String>, // basically, Option<String> is string? in ts
    pub title: String,
    pub description: Option<String>,
    // pub url: String,
    // pub url_to_image: Option<String>,
    // pub published_at: String,
    // pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseArticle {
    pub articles: Vec<Article>
}

