use std::collections::HashSet;

use anyhow::Ok;
use chrono::{DateTime, Utc};
use diesel::{PgConnection, BelongingToDsl, QueryDsl, RunQueryDsl, SelectableHelper, insert_into, ExpressionMethods, BoolExpressionMethods, GroupedBy, PgTextExpressionMethods};
use itertools::Itertools;
use crate::{schema::{news, themes, sources, sourcethemes}, api::models::{SearchQuery, SourceThemePatch}};

use super::models::*;


pub fn get_news(start_date: Option<DateTime<Utc>>, amount: i64, prefs: &SearchQuery, conn: &mut PgConnection) -> Vec<NewsFull> {
    // todo!();
    let start_date = start_date.unwrap_or(chrono::Utc::now());
    let mut query = news::table
        .left_join(sourcethemes::table.left_join(themes::table))
        .left_join(sources::table)
        .into_boxed();
    for pref in &prefs.add_source {
        query = query.or_filter(sources::name.eq(pref));
    }
    for pref in &prefs.remove_source {
        query = query.filter(sources::name.ne(pref));
    }
    for pref in &prefs.add_themes {
        query = query.or_filter(themes::theme_name.eq(pref))
    }
    for pref in &prefs.remove_themes {
        query = query.filter(themes::theme_name.ne(pref))
    }
    if let Some(search) = &prefs.query {
        if search!="" {
            query = query.filter(news::header.ilike(format!("%{search}%")))
        }
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

// pub fn get_sources_by_type(type_val: &str, conn: &mut PgConnection) -> Result<Vec<Source>,anyhow::Error> {
//     let res: Vec<Source> = sources::table
//         .filter(sources::source_type.eq(type_val))
//         .select(Source::as_select())
//         .load::<Source>(conn)?;
//     Ok(res)
// }

pub fn get_all_themes_db(conn: &mut PgConnection) -> Result<Vec<Theme>,anyhow::Error> {
    let res = themes::table
        .order_by(themes::id)
        .select(Theme::as_select())
        .get_results(conn)?;
    Ok(res)
}

pub fn get_sources_db(id0: Option<i32>, amount: i64, conn: &mut PgConnection) -> Result<Vec<Source>,anyhow::Error>  {
    let id0 = id0.unwrap_or(0);
    let res = sources::table
        .filter(sources::id.gt(id0))
        .order_by(sources::id.asc())
        .limit(amount)
        .select(Source::as_select())
        .get_results::<Source>(conn)?;
    Ok(res)
}

pub fn update_sources_db(sources: Vec<Source>, conn: &mut PgConnection) -> Result<(),anyhow::Error> {
    for source in sources {
        diesel::update(sources::table)
            .set(source)
            .execute(conn)?;
    }
    Ok(())
}

pub fn insert_sources_db(sources: Vec<SourceInsert>, conn: &mut PgConnection) -> Result<(),anyhow::Error> {
    diesel::insert_into(sources::table)
        .values(sources)
        .execute(conn)?;
    Ok(())
}

pub fn delete_sources_db(source_ids: Vec<i32>, conn: &mut PgConnection) -> Result<(),anyhow::Error> {
    diesel::delete(sources::table.filter(sources::id.eq_any(source_ids)))
        .execute(conn)?;
    Ok(())
}

pub fn update_source_themes_db(data: Vec<SourceThemePatch>, conn: &mut PgConnection) -> Result<(),anyhow::Error> {
    let themes_names: Vec<&String> = data.iter().map(|x| &x.theme).unique().collect();
    // println!("Темы: {:?}",themes_names);
    let mut themes: Vec<Theme> = themes::table
        .filter(themes::theme_name.eq_any(themes_names))
        .select(Theme::as_select())
        .get_results::<Theme>(conn)?;
    // println!("Темы из таблицы: {:?}",themes);
    let not_existant_themes: Vec<ThemeInsert> = data.iter()
        .filter(|x| !themes.iter().any(|y| x.theme==y.theme_name))
        .cloned().map(|x| x.theme).unique().map(|x| x.into()).collect_vec();
    // println!("Не сущ темы: {:?}",not_existant_themes);
    if !not_existant_themes.is_empty() {
        let mut another = insert_into(themes::table)
            .values(not_existant_themes)
            .returning(Theme::as_returning())
            .get_results(conn)?;
        themes.append(&mut another);
        // println!("Темы обновленные: {:?}",themes);
    }
    let ids_vec = data.iter()
        .map(|x| (x,themes.iter().find_or_first(|theme| theme.theme_name==x.theme)))
        .filter(|x| x.1.is_some())
        .map(|x| (x.0,x.1.unwrap()))
        .map(|x| (x.0.id,x.1.id)).collect_vec();
    for item in ids_vec {
        diesel::update(sourcethemes::table)
            .filter(sourcethemes::id.eq(item.0))
            .set(sourcethemes::theme_id.eq(item.1))
            .execute(conn)?;
    }
        Ok(())
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

