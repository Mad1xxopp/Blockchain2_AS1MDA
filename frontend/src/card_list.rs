use reqwest::Client;
use yew::{Component, Context, Html, html, InputEvent};
use yew::html::Scope;
use crate::card::Card;
use crate::props::{Article, Props};
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;



pub struct CardList {
    news: Vec<Article>,
    link: Scope<CardList>,
    query: String,
}

pub enum Msg {
    FetchReady(Result<Vec<Article>, reqwest::Error>),
    Fetch,
    SetNews(Vec<Article>),
    FetchError,
    SetQuery(String),
    Search,
}


impl Component for CardList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        link.send_message(Msg::Fetch);
        Self {
            news: vec![],
            link,
            query: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                let link = self.link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client = Client::new();
                    let response = client.get("http://localhost:4000/news")
                        .send()
                        .await
                        .unwrap()
                        .json::<Vec<Article>>()
                        .await;
                    link.send_message(Msg::FetchReady(response));
                });
                true
            }
            Msg::FetchReady(response) => {
                if let Ok(articles) = response {
                    self.news = articles;
                }
                true
            }
            Msg::SetQuery(query) => {
                self.query = query;
                false
            }
            Msg::Search => {
                let query = self.query.clone();
                ctx.link().send_future(async move {
                    match Client::new()
                        .get(&format!("http://localhost:4000/search?query={}", query))
                        .send()
                        .await
                    {
                        Ok(resp) => Msg::SetNews(resp.json().await.unwrap_or_default()),
                        Err(_) => Msg::FetchError,
                    }
                });
                false
            }
            Msg::SetNews(news) => {
                self.news = news;
                true
            }
            Msg::FetchError => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input = e.target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>();
            Msg::SetQuery(input.value())
        });

        html! {
            <div>
                <h1 style="text-align: center; color: #333; font-size: 3em; margin: 0; padding: 20px; background-color: #fff; border-radius: 10px; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);">{"Cryptocurrency News Aggregator"}</h1>
                <div class="search-bar" style="display: flex; gap: 10px; margin: 20px auto; max-width: 800px; padding: 0 20px;">
                    <input 
                        type="text" 
                        placeholder="Search by crypto name or symbol (e.g. bitcoin, BTC)" 
                        {oninput}
                        value={self.query.clone()}
                        style="flex: 1; padding: 12px 15px; font-size: 16px; border: 2px solid #e0e0e0; border-radius: 8px; outline: none; transition: border-color 0.3s;"
                    />
                    <button 
                        onclick={ctx.link().callback(|_| Msg::Search)}
                        style="
                            padding: 12px 24px;
                            background-color: #4CAF50;
                            color: white;
                            border: none;
                            border-radius: 8px;
                            font-size: 16px;
                            font-weight: 600;
                            cursor: pointer;
                            transition: background-color 0.3s, transform 0.2s;
                            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                            "
                    >
                        {"Search"}
                    </button>
                </div>
                <div class="news-list" style="display: flex; flex-direction: column; overflow-y: scroll; height: 100vh;">
                    { for self.news.iter().map(|news| html! {
                        <Card
                            image_url={news.urlToImage.clone().unwrap_or_default()}
                            title={news.title.clone().unwrap_or_default()}
                            published_at={news.publishedAt.clone().unwrap_or_default()}
                            author={news.author.clone().unwrap_or_default()}
                            description={news.description.clone().unwrap_or_default()}
                            url={news.url.clone().unwrap_or_default()}
                        />
                    }) }
                </div>
            </div>
        }
    }
}