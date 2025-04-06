# 📰 Cryptocurrency News Aggregator in Rust

A Rust-based web service that aggregates and displays the latest cryptocurrency news from multiple sources. Users can search by cryptocurrency name or symbol to view relevant news articles in a simple, user-friendly interface.

## 📌 Objective

Build a lightweight and efficient cryptocurrency news aggregator using Rust. The system fetches real-time news data from multiple public APIs and displays it to the user.

---

## 🛠 Technology Stack

- **Backend**: Rust (using crates like `reqwest`, `tokio`, `serde`, `actix-web`)
- **Frontend**: Rust-based frontend framework like [Yew](https://yew.rs/)
- **Data Sources**:
  - [News API](https://newsapi.org/)
  - [CoinMarketCap](https://coinmarketcap.com/)
- **(Optional) Caching & Storage**:
  - Redis

---

## ✨ Core Features

- 🔍 Search news by cryptocurrency **name** or **symbol**
- 🔁 Fetch real-time data from **multiple APIs**
- 🗞 Display **title**, **source**, **date**, and **summary** for each article
- ❗ Handle API **errors** and **rate limits**
- 🌐 Simple and intuitive **web interface** for user interaction

---

## 🚀 Getting Started

- Daniarbek Madiyar
- Zholdybayev Didar
- Zhumabayev Alibek
- 
### Prerequisites

- Rust & Cargo installed (`rustup`)
- Optional: Redis if using caching or persistent storage

### Installation

```bash
git clone https://github.com/your-username/crypto-news-aggregator.git
cd crypto-news-aggregator
cargo build
cargo run
