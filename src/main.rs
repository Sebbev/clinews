use colour::{blue_ln, green_ln, grey_ln};
use dotenv::dotenv;
use std::error::Error;

use newsapi::{Article, Country, Endpoint, NewsAPI};

fn render_articles(articles: &Vec<Article>) {
    let underline_start = "\x1B[4m";
    let underline_end = "\x1B[0m";
    let italic_start = "\x1B[3m";
    let italic_end = "\x1B[0m";

    for a in articles {
        green_ln!("> {}", a.title());
        blue_ln!("- {}{}{}", underline_start, a.url(), underline_end);
        grey_ln!("- {}{}{}", italic_start, a.source().name(), italic_end);
        println!();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[allow(unused_must_use)]
    {
        dotenv();
    }
    let api_key = std::env::var("NEWSAPI_KEY")?;

    let mut news_api = NewsAPI::new(&api_key);
    news_api
        .endpoint(Endpoint::TopHeadlines)
        .country(Country::SE);

    let news_api_response = news_api.fetch_async().await?;

    render_articles(&news_api_response.articles());

    Ok(())
}
