use diesel::{associations::{Associations, Identifiable}, prelude::Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Selectable, Identifiable, Queryable,Debug,Serialize,Deserialize)]
#[diesel(table_name = crate::schema::feeds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Feed {
	pub id: i32,
	pub user_id: i32,
	pub name: String
}

#[derive(Insertable, Debug,Serialize,Deserialize)]
#[diesel(table_name = crate::schema::feeds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FeedInsert {
	pub user_id: i32,
	pub name: String
}

#[derive(Selectable, Identifiable, Associations,  Queryable,Debug,Serialize,Deserialize, Clone)]
#[diesel(table_name = crate::schema::feedsource)]
#[diesel(belongs_to(Feed))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FeedSource {
	pub id: i32,
	pub feed_id: i32,
	pub source_theme_id: i32
}
