use std::collections::HashSet;

use anyhow::Ok;
use diesel::{PgConnection, BelongingToDsl, QueryDsl, RunQueryDsl, SelectableHelper, insert_into, ExpressionMethods, BoolExpressionMethods, GroupedBy};
use itertools::Itertools;
use crate::schema::{news, themes, sources, sourcethemes};

use super::models::*;


pub fn get_news(max_id: i32, amount: i64, conn: &mut PgConnection) -> Vec<NewsFull> {
    // todo!();
    news::table
        .left_join(sourcethemes::table.left_join(themes::table))
        .left_join(sources::table)
        .filter(news::id.le(max_id))
        .order_by(news::id.desc())
        .limit(amount)
        .select((NewEntry::as_select(),Option::<Theme>::as_select(),Option::<Source>::as_select()))
        .load::<(NewEntry,Option<Theme>,Option<Source>)>(conn)
        .unwrap_or_default()
        .into_iter().filter_map(|x| x.try_into().ok()).collect_vec()
}

pub fn get_sources_by_type(type_val: &str, conn: &mut PgConnection) -> Result<Vec<Source>,anyhow::Error> {
    let res: Vec<Source> = sources::table
        .filter(sources::source_type.eq(type_val))
        .select(Source::as_select())
        .load::<Source>(conn)?;
    Ok(res)
}

pub fn get_sources_and_last_entry_by_type(type_val: &str, conn: &mut PgConnection) -> Result<Vec<(Source,Option<NewEntry>)>,anyhow::Error> {
    let sources: Vec<Source> = sources::table
        .filter(sources::source_type.eq(type_val))
        .select(Source::as_select())
        .load::<Source>(conn)?;

    let last_entries: Vec<NewEntry> = NewEntry::belonging_to(&sources)
        .select(NewEntry::as_select())
        .order_by((news::source_id.desc(),news::date_time.desc()))
        .distinct_on(news::source_id)
        .load::<NewEntry>(conn)?;
    
    let res = last_entries
        .grouped_by(&sources)
        .into_iter()
        .zip(sources)
        .map(|(news, source)| (source, news.into_iter().next()))
        .collect::<Vec<(Source,Option<NewEntry>)>>();
    Ok(res)
}


pub fn add_news_db(news_vec: Vec<NewsInsert>, conn: &mut PgConnection) -> Result<Vec<NewEntry>,anyhow::Error> {
    let source_theme_info = news_vec.iter()
        .map(|x| (x.source_id,x.theme_source.as_str())).collect_vec();
    let source_theme_info: HashSet<(i32, &str)> = HashSet::from_iter(source_theme_info);
    // println!("{source_theme_info:?}");
    let source_themes = get_source_themes_with_def_insert(source_theme_info, conn)?;
    // println!("{source_themes:?}");
    let news_tuple: Vec<NewsDBInsert> = news_vec.iter()
        .map(|x| (x,source_themes.iter().find_or_first(|y| y.source_theme_name == x.theme_source)
            .expect("themes already should be created and returned"))
        ).unique()
        .map(|x| x.into())
        .collect_vec();
    // println!("{news_tuple:?}");
    let res = insert_into(news::table)
        .values(news_tuple)
        .returning(NewEntry::as_returning())
        .load::<NewEntry>(conn)?;
    Ok(res)
}

pub fn get_source_themes_with_def_insert<'a>(mut source_theme_info: HashSet<(i32, &'a str)>,conn: &mut PgConnection) -> anyhow::Result<Vec<SourceTheme>> {
    let mut query = sourcethemes::table.into_boxed();
    for (id,name) in source_theme_info.iter() {
        query = query.or_filter(sourcethemes::source_id.eq(id).and(sourcethemes::source_theme_name.eq(name)))
    }
    let mut db_themes = query.select(SourceTheme::as_select())
        .load::<SourceTheme>(conn)?;
    let db_themes_set: HashSet<(i32,&str)> = HashSet::from_iter(db_themes.iter().map(|x| (x.source_id, x.source_theme_name.as_str())));
    // println!("{db_themes_set:?}");
    source_theme_info.retain(|x| !db_themes_set.contains(x));
    // println!("{source_theme_info:?}");

    let mut to_add: Vec<SourceTheme> = if !source_theme_info.is_empty() {
        let values: Vec<SourceThemeInsert> = source_theme_info.into_iter().map(|x| x.into()).collect_vec(); 
        // println!("{values:?}");
        insert_into(sourcethemes::table)
            .values(values)
            .returning(SourceTheme::as_returning())
            .load::<SourceTheme>(conn)?
    } else {vec![]};
    db_themes.append(&mut to_add);
    Ok(db_themes)
}



#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::db::establish_connection;
    use super::{get_source_themes_with_def_insert, get_news};
    
    #[test]
    fn get_source_themes_with_def_insert_test() {
        let mut conn = establish_connection().expect("db conn")
            .get().expect("db conn 2");
        let set: HashSet<(i32,&str)> = HashSet::from_iter(vec![(1,"Politics"),(2,"Sport")]);
        let res = get_source_themes_with_def_insert(set, &mut conn).expect("Error in get_source_themes");
        // println!("{res:?}");
    }

    #[test]
    fn get_news_test() {
        let mut conn = establish_connection().expect("db conn")
            .get().expect("db conn 2");
        let res = get_news(0,6, &mut conn);
        println!("{res:?}")
        // println!("{res:?}");
    }

}