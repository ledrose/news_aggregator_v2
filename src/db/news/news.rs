use anyhow::anyhow;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, Insertable, insert_into, ExpressionMethods};
use itertools::Itertools;
use crate::{schema::{news, themes, sources, sourcethemes}, error::{self, ApiError}};

use super::models::*;


pub fn get_news(id0: i64, amount: i64, conn: &mut PgConnection) -> Vec<NewsFull> {
    news::table
        .left_join(themes::table)
        .left_join(sources::table)
        .order_by(news::id)
        .offset(id0)
        .limit(amount)
        .select((NewEntry::as_select(),Option::<Theme>::as_select(),Option::<Source>::as_select()))
        .load::<(NewEntry,Option<Theme>,Option<Source>)>(conn)
        .unwrap_or_default()
        .into_iter().filter_map(|x| x.try_into().ok()).collect_vec()
}


pub fn add_news_db(news_vec: Vec<NewsInsert>, conn: &mut PgConnection) -> Result<(),anyhow::Error> {
    // let source_id = sources::table
    //     .select(Source::as_select())
    //     .filter(sources::name.eq())
        
    // insert_into(sourcethemes::table)
    //     .values(sourcethemes::source_id)
    
    // use crate::schema::news::dsl::*;
    //     insert_into(news)
    //     .values(news_vec)
    //     .execute(conn)
    //     .map_err(|_| anyhow!("Error in inserting news"))?;
    Ok(())
}