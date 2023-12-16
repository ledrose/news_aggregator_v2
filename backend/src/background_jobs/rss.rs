use anyhow::Ok;
use chrono::DateTime;
use rss::Channel;
use crate::db::news::{news::{get_sources_and_last_entry_by_type, add_news_db, get_sources_by_type}, models::{Source, NewsInsert, NewEntry}};

impl NewsInsert {
    pub fn from_rss_item(value: &rss::Item, source_id: i32) -> NewsInsert {
        NewsInsert {
            header: value.title.clone().unwrap_or("Not title".to_string()),
            source_id, 
            theme_source: value.categories[0].name.clone(),
            text: value.link.clone().unwrap_or("Has no link".to_string()),
            date_time: DateTime::parse_from_rfc2822(value.pub_date.as_ref()
                    .unwrap_or(&String::from("")).as_str()
                ).unwrap_or_default().into(),
        }
    }
}


pub struct RssTask;

impl RssTask {
    // pub fn get_interval() -> actix_rt::time::Interval {
    //     time::interval(Duration::from_secs(10*60))
    // }

    pub async fn update(conn: &mut diesel::prelude::PgConnection) -> Result<(),anyhow::Error> {
        let source_info =  get_sources_and_last_entry_by_type("rss", conn)?;
        let mut news = vec![];
        for source in source_info {
            if let (Source { id, link: Some(link), .. }, entry) = &source {
                let content = reqwest::get(link)
                    .await?
                    .bytes()
                    .await?;
                let channel = Channel::read_from(&content[..])?;
                // println!("Channel items: {:?}",channel.items());
                for item in channel.items() {
                    if let Some(entry) = entry {
                        if item.title == Some(entry.header.clone()) {
                            break;
                        }   
                    }
                    let news_entry = NewsInsert::from_rss_item(item, *id);
                    news.push(news_entry);
                }
            }
        }
        let _ = add_news_db(news, conn)?;
        Ok(())
    }
}

