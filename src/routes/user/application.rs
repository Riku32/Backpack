use actix_web::{get, http::StatusCode, web, Responder, Scope};
use sea_orm::{prelude::*, Condition};

use crate::{
    database::entity::applications,
    internal::auth::{auth_role, Auth},
    models::application::*,
    services::{application::ApplicationService, prelude::DataService, ToPageResponse},
};

pub fn get_routes() -> Scope {
    web::scope("/application").service(list)
}

/// Get all applications owned by a user.
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
	context_path = "/api/user/{user_id}/application",
	tag = "application",
	responses((status = 200, body = ApplicationPage)),
	params(
		("page_number" = u64, Path, description = "Page to get applications by (starts at 1)"),
		("user_id" = str, Path)
	),
	security(("apiKey" = [])),
)]
#[get("/{page_number}")]
async fn list(
    service: web::Data<ApplicationService>,
    params: web::Path<(String, usize)>,
    user: Auth<auth_role::User>,
) -> impl Responder {
    let (user_id, page_number) = params.to_owned();

    service
        .get_page_authorized(
            page_number,
            5,
            Some(
                Condition::any().add(applications::Column::UserId.eq(if user_id == "@me" {
                    user.id.to_owned()
                } else {
                    user_id.to_owned()
                })),
            ),
            &user_id,
            &user,
        )
        .await
        .to_page_response::<ApplicationData>(StatusCode::OK)
}
