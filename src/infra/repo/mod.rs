use std::error::Error;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::{ExpressionMethods, JoinOnDsl, MysqlConnection, QueryDsl, RunQueryDsl, TextExpressionMethods};
use diesel::query_dsl::limit_dsl::LimitDsl;
use crate::domain::model::{FootStamp, QueryError, User};
use crate::schema::foot_stamp;
use crate::schema::foot_stamp::latitude;
use crate::schema::user::dsl::*;

pub fn get_user_count(conn: &MysqlConnection) -> Result<i64, QueryError> {
    let result = user.filter(
        crate::schema::user::dsl::created_at.between(
            NaiveDateTime::new(NaiveDate::from_ymd(1999,1,1), NaiveTime::from_hms(0,0,0)),
            NaiveDateTime::new(NaiveDate::from_ymd(2005,1,1), NaiveTime::from_hms(0,0,0))))
        .filter(description.like("%a%"))
        .left_join(crate::schema::foot_stamp::dsl::foot_stamp)
        .filter(crate::schema::foot_stamp::latitude.between(-33.35123421, 130.012354123))
        .filter(crate::schema::foot_stamp::longitude.between(-93.414312513, 19.4213))
        .left_join(crate::schema::post::dsl::post)
        .filter(crate::schema::post::published.eq(true))
        .filter(crate::schema::post::body.like("%s%"))
        .count()
        .first(conn)?;

    Ok(result)
}