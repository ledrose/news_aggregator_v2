pub mod rss;
use actix_rt::time::Interval;

use crate::db::DBPool;

pub async fn start_background_tasks(pool: DBPool,mut interval: Interval) {
    actix_rt::spawn(async move {  
        let mut conn = pool.get().expect("Background tasks failed to connect to db");
        loop {
            interval.tick().await;
            rss::update_rss_sources();
        }
    });
}

