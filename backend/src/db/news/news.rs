use std::collections::HashSet;

use anyhow::Ok;
use chrono::{DateTime, Utc};
use diesel::{dsl::not,PgConnection, BelongingToDsl, QueryDsl, RunQueryDsl, SelectableHelper, insert_into, ExpressionMethods, BoolExpressionMethods, GroupedBy};
use itertools::Itertools;
use crate::{schema::{news, themes, sources, sourcethemes}, api::models::{Preferences, PreferenceType, PreferenceAction}};

use super::models::*;


pub fn get_news(start_date: Option<DateTime<Utc>>, amount: i64, prefs: Vec<Preferences>, conn: &mut PgConnection) -> Vec<NewsFull> {
    // todo!();
    let start_date = start_date.unwrap_or(chrono::Utc::now());
    let mut query = news::table
        .left_join(sourcethemes::table.left_join(themes::table))
        .left_join(sources::table)
        .into_boxed();
    for pref in prefs {
        query = match pref.pref_type {
                PreferenceType::Source { name } => {
                    match &pref.action {
                        PreferenceAction::Add => {query.or_filter(sources::name.eq(name))}
                        PreferenceAction::Remove => {query.filter(sources::name.ne(name))},
                    }
                },
                PreferenceType::Theme { name } => {
                    match &pref.action {
                        PreferenceAction::Add => {query.or_filter(themes::theme_name.eq(name))}
                        PreferenceAction::Remove => {query.filter(themes::theme_name.ne(name))},
                    }
                }
        };
    }
    query
        .filter(news::date_time.lt(start_date))
        .order_by(news::date_time.desc())
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

