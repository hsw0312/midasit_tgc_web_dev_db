use crate::member::dto::member::Member;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MembersResponseData {
    pub members: Vec<Member>,
}
