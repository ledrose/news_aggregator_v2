use std::collections::HashSet;

use anyhow::Ok;
use diesel::{dsl::max, insert_into, BelongingToDsl, BoolExpressionMethods, ExpressionMethods, GroupedBy, PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use itertools::Itertools;
use crate::{api::models::{SearchQuery, SourceThemePatch}, schema::{news, relevance_score, sources, sourcethemes, themes}};

use super::models::*;


pub fn get_news(mut max_news_id: Option<i32>, batch_offset: i64, amount: i64, prefs: &mut SearchQuery, conn: &mut PgConnection) -> (Vec<NewsFull>,i32) {
    if max_news_id.is_none() {
        max_news_id = news::table.select(max(news::id)).first::<Option<i32>>(conn).unwrap_or_default();
        if max_news_id.is_none() {
            return (vec![],0);
        }
    }
    let max_id = max_news_id.unwrap();
    let mut query = news::table
        .left_join(sourcethemes::table.left_join(themes::table))
        .left_join(sources::table)
        .into_boxed();
    if !prefs.allowed_sources.is_empty() {
        if prefs.add_source.is_empty() {
            prefs.add_source = prefs.allowed_sources.clone();
        } else {
            prefs.add_source = prefs.add_source.clone().into_iter().filter(|el| prefs.allowed_sources.contains(el)).collect_vec();
        }
    }
    if !prefs.add_source.is_empty() {
        query = query.filter(sources::name.eq_any(&prefs.add_source));
    }
    if !prefs.remove_source.is_empty() {
        query = query.filter(sources::name.ne_all(&prefs.remove_source));
    }
    if !prefs.add_themes.is_empty() {
        query = query.filter(themes::theme_name.eq_any(&prefs.add_themes));
    }
    if !prefs.remove_themes.is_empty() {
        query = query.filter(themes::theme_name.ne_all(&prefs.remove_themes));
    }

    if let Some(start_date) = prefs.start_date {
        query = query.filter(news::date_time.ge(start_date));
    }
    if let Some(end_date) = prefs.end_date {
        // tracing::info!("Start_date {:?}\nEnd_date:{:?}",prefs.start_date,prefs.end_date);
        query = query.filter(news::date_time.le(end_date));
    } else {
        query = query.filter(news::date_time.le(chrono::Utc::now()));
    }
    if let Some(search) = &prefs.query {
        if !search.is_empty() {
            query = query.filter(news::header.ilike(format!("%{search}%")))
        }
    }
    query = query.filter(news::id.le(max_id));

    if let Some(filter) = &prefs.filter {
        match filter {
            crate::api::models::Filter::Date => {
                query = query.order_by(news::date_time.desc());
            },
            crate::api::models::Filter::Title => {
                query = query.order_by(news::header.desc());
            },
            crate::api::models::Filter::SearchResult => {
                if let Some(search) = &prefs.query {
                    if !search.is_empty() {
                        query = query.order_by(relevance_score(search,news::header).desc())

                    } else {
                        query = query.order_by(news::date_time.desc());
                    }
                } else {
                    query = query.order_by(news::date_time.desc());
                }
            },
        }
    } else {
        query = query.order_by(news::date_time.desc());
    }

    let vec = query
        // .order_by(news::date_time.desc())
        .offset(batch_offset)
        .limit(amount)
        .select((NewEntry::as_select(),Option::<Theme>::as_select(),Option::<Source>::as_select()))
        .load::<(NewEntry,Option<Theme>,Option<Source>)>(conn)
        .unwrap_or_default()
        .into_iter().filter_map(|x| x.try_into().ok()).collect_vec();
    (vec,max_id)
}


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

pub fn get_source_themes_with_def_insert(mut source_theme_info: HashSet<(i32, &str)>,conn: &mut PgConnection) -> anyhow::Result<Vec<SourceTheme>> {
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

