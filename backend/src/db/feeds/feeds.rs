use diesel::{insert_into, BelongingToDsl, BoolExpressionMethods, ExpressionMethods, GroupedBy, PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{api::models::FeedInfo, db::feeds::models::{Feed, FeedInsert, FeedSource}, schema::{feeds, feedsource, sources, sourcethemes, themes}};


pub fn add_feed(user_id: i32, feed: FeedInfo, conn: &mut PgConnection) ->  Result<(),anyhow::Error> {
	// let feed_id = diesel::insert_into(feeds::table).values(FeedInsert { user_id, name: feed.name }).returning(feeds::id).first::<i32>(conn)?;
	// let _ = diesel::insert_into(feedsource::table).values(records)

	todo!()
}

pub fn get_feeds(user_id: i32, conn: &mut PgConnection) -> Result<Vec<FeedInfo>,anyhow::Error> {
	let feeds: Vec<Feed> = feeds::table.filter(feeds::user_id.eq(user_id)).select(Feed::as_select()).load::<Feed>(conn)?;
	let feed_sources: Vec<(FeedSource,String,String)> = FeedSource::belonging_to(&feeds)
		.inner_join(sourcethemes::table.inner_join(themes::table).inner_join(sources::table))
		.select((FeedSource::as_select(),themes::theme_name,sources::name))
		.order_by(feedsource::feed_id)
		.distinct_on(feedsource::feed_id)
		.load::<(FeedSource,String,String)>(conn)?;
	let res = feed_sources
        .grouped_by(&feeds)
        .into_iter()
        .zip(feeds)
        .map(|x| x.into())
        .collect::<Vec<FeedInfo>>();
	Ok(res)
}

pub fn delete_feed(user_id: i32, feed_name: &String, conn: &mut PgConnection) -> Result<(),anyhow::Error> {
	let id: i32 = feeds::table.select(feeds::id).filter(
		feeds::name.eq(feed_name).and(
			feeds::user_id.eq(user_id))
	).get_result::<i32>(conn)?;
	diesel::delete(feedsource::table.filter(feedsource::feed_id.eq(id))).execute(conn)?;
	diesel::delete(feeds::table.filter(feeds::id.eq(id))).execute(conn)?;
	Ok(())
}