pub mod rss;
use actix_rt::time::Interval;

use crate::db::DBPool;

use self::rss::RssTask;

pub async fn start_background_tasks(pool: DBPool,mut interval: Interval) {
    actix_rt::spawn(async move {  
        let mut conn = pool.get().expect("Background tasks failed to connect to db");
        loop {
            interval.tick().await;
            if let Err(err)=  RssTask::update(&mut conn).await {
                log::error!("Error in RssTask: {err:?}");
            } else {
                log::info!("RssTask completed");
            };
        }
    });
}


// trait BackgroundTask {
//     fn get_interval() -> Interval;
//     fn update(conn: &mut PgConnection);
// }