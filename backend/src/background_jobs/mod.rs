pub mod rss;


use deadpool_diesel::postgres::Pool;
use tokio::time::Interval;
use tracing::{event, Level};

use self::rss::RssTask;

pub async fn start_background_tasks(pool: Pool,mut interval: Interval) {
    tokio::spawn(async move {  
        loop {
            interval.tick().await;
            if let Err(err) =  RssTask::update(&pool).await {
                event!(Level::INFO,"Error in RssTask: {err:?}");
            } else {
                event!(Level::INFO,"RssTask completed");
            };
        }
    });
}


// trait BackgroundTask {
//     fn get_interval() -> Interval;
//     fn update(conn: &mut PgConnection);
// }