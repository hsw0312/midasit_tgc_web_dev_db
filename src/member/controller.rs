use crate::member::service;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/members")]
pub async fn get_members(data: web::Data<mysql::Pool>) -> actix_web::Result<impl Responder> {
    let members = web::block(move || service::get_all_member(&data)).await??;
    Ok(web::Json(members))
}

#[post("/member")]
pub async fn post_member(
    data: web::Data<mysql::Pool>,
    member: web::Json<crate::member::dto::member::Member>,
) -> actix_web::Result<impl Responder> {
    web::block(move || service::create_member(&data, member.into_inner())).await??;
    Ok(HttpResponse::Created())
}