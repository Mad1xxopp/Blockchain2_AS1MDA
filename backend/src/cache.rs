use redis::{Client, RedisResult, Commands}; // Add `Commands` trait here
use crate::model::Article;
use serde_json; // Don't forget to import serde_json

pub struct Cache {
    client: Client
}

impl Cache {
    pub fn new() -> Cache {
        let client = Client::open("redis://127.0.0.1/")
            .unwrap();

        Cache {
            client
        }
    }

    pub fn store_news(& mut self, news: Vec<Article>) -> RedisResult<()> {
        let key = String::from("LATEST_NEWS");

        let mut con = self.client
            .get_connection()?;

        let data = serde_json::to_string(&news).unwrap();

        con.set_ex(key, data, 600)
    }

    pub fn get_news(& mut self) -> Option<String>{
        let key = String::from("LATEST_NEWS");
        
        let mut con = self.client.get_connection()
            .unwrap();

        let value: Option<String> = con.get(key)
            .unwrap();
        value
    }
    pub fn search_news(&mut self, query: &str) -> RedisResult<Option<Vec<Article>>> {
        let mut con = self.client.get_connection()?;
        let news: Option<String> = con.get("LATEST_NEWS")?;
        
        Ok(news.and_then(|data| {
            let articles: Vec<Article> = serde_json::from_str(&data).ok()?;
            let filtered = articles.into_iter()
                .filter(|a| {
                    a.title.as_ref().map_or(false, |t| t.contains(query)) ||
                    a.description.as_ref().map_or(false, |d| d.contains(query))
                })
                .collect();
            Some(filtered)
        }))
    }
}