use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use itertools::Itertools;
use crate::schema::{news, themes, sources};

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
