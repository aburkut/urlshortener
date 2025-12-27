use diesel::QueryResult;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::*;
use crate::schema::*;

pub struct UrlRepository {}

impl UrlRepository {
    pub async fn find_by_id(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Url> {
        urls::table.find(id).get_result(c).await
    }

    pub async fn find_by_short(c: &mut AsyncPgConnection, short: &str) -> QueryResult<Url> {
        urls::table.filter(urls::short.eq(short)).first(c).await
    }

    pub async fn find_by_url(c: &mut AsyncPgConnection, url: &str) -> QueryResult<Url> {
        urls::table
            .filter(urls::url.eq(url))
            .filter(urls::expires_at.is_null().or(urls::expires_at.gt(diesel::dsl::now)))
            .order(urls::created_at.desc())
            .first(c)
            .await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_url: NewUrl) -> QueryResult<Url> {
        diesel::insert_into(urls::table)
            .values(new_url)
            .get_result(c)
            .await
    }

    pub async fn increment_clicks(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Url> {
        diesel::update(urls::table.find(id))
            .set(urls::clicks.eq(urls::clicks + 1))
            .get_result(c)
            .await
    }
}