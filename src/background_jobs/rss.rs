use std::time::Duration;

use actix_rt::time;
use anyhow::Ok;
use rss::Channel;
use crate::db::news::{news::{get_sources_by_type, add_news_db}, models::{Source, NewsInsert}};

impl NewsInsert {
    pub fn from_rss_item(value: &rss::Item, source_id: i32) -> NewsInsert {
        NewsInsert {
            header: value.title.clone().unwrap_or("Not title".to_string()),
            source_id, 
            theme_source: value.categories[0].name.clone(),
            text: value.link.clone().unwrap_or("Has no link".to_string()),
        }
    }
}


pub struct RssTask;

impl RssTask {
    // pub fn get_interval() -> actix_rt::time::Interval {
    //     time::interval(Duration::from_secs(10*60))
    // }

    pub async fn update(conn: &mut diesel::prelude::PgConnection) -> Result<(),anyhow::Error> {
        let source_info =  get_sources_by_type("rss", conn)?;
        let mut news = vec![];
        for source in source_info {
            if let Source { id, name, source_type: Some(source_type), link: Some(link) } = &source {
                let content = reqwest::get(link)
                    .await?
                    .bytes()
                    .await?;
                let channel = Channel::read_from(&content[..])?;
                // println!("{:?}",channel.);
                for item in channel.items() {
                    let news_entry = NewsInsert::from_rss_item(item, *id);
                    news.push(news_entry);
                }
            }
        }
        let _ = add_news_db(news, conn)?;
        Ok(())
    }
}

