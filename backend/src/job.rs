use std::time::Duration;
use dotenv::dotenv;
use std::env;
use serde_json::Value;
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::cache::Cache;
use crate::model::{ApiResponse, Article};

async fn fetch_news() -> Vec<Article> {
    dotenv().ok();

    let key = env::var("API_KEY")
        .unwrap();

        let terms = vec![
            "cryptocurrency",
        ];

        let query = terms.join(" OR ");
        let url = format!(
            "https://newsapi.org/v2/everything?q={}&sortBy=publishedAt&language=en&apiKey={}", 
            query, key
        );


    let client = reqwest::Client::new();

    let response = client.get(url)
        .header("User-Agent", "News-Feed-App")
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await.unwrap();

    println!("{:?}", response);

    let api_response: ApiResponse = serde_json::from_value(response).unwrap();

    let news: Vec<Article> = api_response.articles.into_iter()
        .map(|article| {
            Article {
                title: article.title,
                description: article.description,
                author: article.author,
                publishedAt: article.publishedAt,
                url: article.url,
                urlToImage: article.urlToImage,
            }
        }).collect();

    news
}



pub async fn run() {
    let scheduler = JobScheduler::new()
        .await
        .unwrap();

    let job = Job::new_repeated_async(Duration::from_secs(30), |_uuid, _l| {

        Box::pin(
            async {
                let news = fetch_news()
                    .await;

                println!("Get News {:?}", news.iter().count());
                let mut cache = Cache::new();
                cache.store_news(news).unwrap()
            }
        )

    }).unwrap();

    scheduler.add(job).await.unwrap();
    scheduler.start().await.unwrap();
}