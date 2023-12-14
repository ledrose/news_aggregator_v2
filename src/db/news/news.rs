use std::collections::HashSet;

use anyhow::anyhow;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, Insertable, insert_into, ExpressionMethods, BoolExpressionMethods};
use itertools::Itertools;
use crate::{schema::{news, themes, sources, sourcethemes::{self, source_theme_name}}, error::{self, ApiError}};

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


pub fn add_news_db(news_vec: Vec<NewsInsert>, conn: &mut PgConnection) -> Result<Vec<NewEntry>,anyhow::Error> {
    let source_names: HashSet<&str> = HashSet::from_iter(news_vec.iter().map(|x| x.source.as_str()));
    let sources = get_sources_with_def_insert(source_names, conn)?;
    // println!("{sources:?}");
    // This is very ineffective as it is basically O(n*m) instead of O(n). Can be changed to db call as all sources already created/
    let news_with_source = news_vec.iter()
        .map(|x| (x,sources.iter().find_or_first(|y| x.source.as_str() == y.name)
            .expect("sources already should be created and returned"))
        ).unique().collect_vec();
    let source_theme_info = news_with_source.iter()
        .map(|x| (x.1.id,x.0.theme_source.as_str())).collect_vec();
    let source_theme_info: HashSet<(i32, &str)> = HashSet::from_iter(source_theme_info);
    // println!("{source_theme_info:?}");
    let source_themes = get_source_themes_with_def_insert(source_theme_info, conn)?;
    // println!("{source_themes:?}");
    let news_tuple: Vec<NewsDBInsert> = news_with_source.iter()
        .map(|x| (x.0,x.1,source_themes.iter().find_or_first(|y| y.source_theme_name == x.0.theme_source)
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

pub fn get_sources_with_def_insert<'a>(mut source_names: HashSet<&'a str>,conn: &mut PgConnection) -> anyhow::Result<Vec<Source>> {
    let mut db_sources: Vec<Source> = sources::table
        .filter(sources::name.eq_any(&source_names))
        .select(Source::as_select())
        .load::<Source>(conn)?;
    let db_sources_set: HashSet<&str> = HashSet::from_iter(db_sources.iter().map(|x| x.name.as_str()));
    source_names.retain(|x| !db_sources_set.contains(x));
    let mut to_add: Vec<Source> = if !source_names.is_empty() {
        let values: Vec<SourceInsert> = source_names.into_iter().map(|x| x.into()).collect_vec(); 
        insert_into(sources::table)
            .values(values)
            .returning(Source::as_returning())
            .load::<Source>(conn)?
    } else {vec![]};
    db_sources.append(&mut to_add);
    Ok(db_sources)
}

pub fn get_source_themes_with_def_insert<'a>(mut source_theme_info: HashSet<(i32, &'a str)>,conn: &mut PgConnection) -> anyhow::Result<Vec<SourceTheme>> {
    let mut query = sourcethemes::table.into_boxed();
    for (id,name) in source_theme_info.iter() {
        query = query.or_filter(sourcethemes::source_id.eq(id).and(sourcethemes::source_theme_name.eq(name)))
    }
    let mut db_themes = query.select(SourceTheme::as_select())
        .load::<SourceTheme>(conn)?;
    let db_themes_set: HashSet<(i32,&str)> = HashSet::from_iter(db_themes.iter().map(|x| (x.source_id, x.source_theme_name.as_str())));
    println!("{db_themes_set:?}");
    source_theme_info.retain(|x| !db_themes_set.contains(x));
    println!("{source_theme_info:?}");

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

    use crate::db::{establish_connection, news::models::NewsInsert};

    use super::{get_sources_with_def_insert, get_source_themes_with_def_insert, get_news, add_news_db};


    #[test]
    fn get_sources_id_with_def_insert_test() {
        let mut conn = establish_connection().expect("db conn")
            .get().expect("db conn 2");

        let set: HashSet<&str> = HashSet::from_iter(vec!["RT","Idiot"]);
        let res = get_sources_with_def_insert(set,&mut conn).expect("Error in get_sources");
        // println!("{res:?}");

    }
    #[test]
    fn get_source_themes_with_def_insert_test() {
        let mut conn = establish_connection().expect("db conn")
            .get().expect("db conn 2");
        let set: HashSet<(i32,&str)> = HashSet::from_iter(vec![(1,"Politics"),(2,"Sport")]);
        let res = get_source_themes_with_def_insert(set, &mut conn).expect("Error in get_source_themes");
        // println!("{res:?}");
    }
    #[test]
    fn insert_news() {
        let mut conn = establish_connection().expect("db conn")
            .get().expect("db conn 2");
        let news_vec = vec![
            NewsInsert { header: "lorum".to_owned(), source: "News Today".to_owned(), theme_source: "Sport".to_owned(), text: "lorum".to_owned() },
            NewsInsert { header: "lorum".to_owned(), source: "RT".to_owned(), theme_source: "Politics".to_owned(), text: "lorum".to_owned() },
            NewsInsert { header: "lorum".to_owned(), source: "RT".to_owned(), theme_source: "Sport".to_owned(), text: "lorum".to_owned() }
        ];
        let res = add_news_db(news_vec, &mut conn).expect("Everything should be fine!");
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