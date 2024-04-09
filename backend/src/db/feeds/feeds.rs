use diesel::{insert_into, BelongingToDsl, BoolExpressionMethods, ExpressionMethods, GroupedBy, PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use itertools::Itertools;

use crate::{api::models::FeedInfo, db::{feeds::models::{Feed, FeedInsert, FeedSource, FeedSourceInsert}, news::models::Source}, schema::{feeds, feedsource, sources, sourcethemes, themes}};


pub fn add_feed(user_id: i32, feed: FeedInfo, conn: &mut PgConnection) ->  Result<(),anyhow::Error> {
	let feed_id = diesel::insert_into(feeds::table).values(FeedInsert { user_id, name: feed.name }).returning(feeds::id).load::<i32>(conn)?;
	if let Some(feed_id) = feed_id.first() {
		let sources: Vec<Source> = sources::table.filter(sources::name.eq_any(&feed.sources))
			.select(Source::as_select())
			.get_results::<Source>(conn)?;
		let feed_sources = sources.into_iter()
			.map(|x| FeedSourceInsert { feed_id: *feed_id, source_id: x.id }).collect_vec();
		diesel::insert_into(feedsource::table).values(feed_sources).execute(conn)?;
	}
	Ok(())
}

pub fn get_feeds(user_id: i32, conn: &mut PgConnection) -> Result<Vec<FeedInfo>,anyhow::Error> {
	let feeds: Vec<Feed> = feeds::table.filter(feeds::user_id.eq(user_id)).select(Feed::as_select()).load::<Feed>(conn)?;
	let feed_sources: Vec<(FeedSource,String)> = FeedSource::belonging_to(&feeds)
		.inner_join(sources::table)
		.select((FeedSource::as_select(),sources::name))
		.order_by(feedsource::feed_id)
		.distinct_on(feedsource::feed_id)
		.load::<(FeedSource,String)>(conn)?;
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