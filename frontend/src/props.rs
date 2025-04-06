use yew::Properties;
use serde::{Deserialize, Serialize};


#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub published_at: String,
    pub author: String,
    pub url: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub publishedAt: Option<String>,
    pub url: Option<String>,
    pub urlToImage: Option<String>,
}
