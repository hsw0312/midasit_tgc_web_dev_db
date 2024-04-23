use crate::db::member::*;
use crate::member::dto::member::Member;
use crate::member::dto::response::MembersResponseData;

use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum MemberError {
    MysqlError(mysql::Error),
    Unknown,
}

impl actix_web::ResponseError for MemberError {
    fn status_code(&self) -> StatusCode {
        match self {
            MemberError::MysqlError(_) | MemberError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn get_all_member(pool: &mysql::Pool) -> Result<MembersResponseData, MemberError> {
    let mut conn = pool.get_conn()?;

    Ok(MembersResponseData {
        members: query::select_members(&mut conn)?
            .iter()
            .map(|table_member| Member {
                name: table_member.name.clone(),
                age: table_member.age,
            })
            .collect::<Vec<Member>>(),
    })
}

pub fn create_member(pool: &mysql::Pool, member: Member) -> Result<(), MemberError> {
    let mut conn = pool.get_conn()?;
    query::post_member(&mut conn, member.name, member.age)?;
    Ok(())
}
