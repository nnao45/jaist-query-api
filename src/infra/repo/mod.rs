use std::error::Error;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::query_dsl::limit_dsl::LimitDsl;
use crate::domain::model::{QueryError, User};
use crate::schema::user::dsl::*;

pub fn get_user_count(conn: &MysqlConnection) -> Result<i64, QueryError> {
    let result = user.filter(
        created_at.between(
            NaiveDateTime::new(NaiveDate::from_ymd(1999,1,1), NaiveTime::from_hms(0,0,0)),
            NaiveDateTime::new(NaiveDate::from_ymd(2005,1,1), NaiveTime::from_hms(0,0,0))))
        .count()
        .first(conn)?;

    Ok(result)
}