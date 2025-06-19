use crate::modules::user::user_controller::{
    __path_get_all_users_use_json, __path_get_all_users_use_struct,
};
use crate::modules::user::user_model::UserData;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_all_users_use_struct,
        get_all_users_use_json,
    ),
    components(
        schemas(UserData)
    ),
    tags(
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;
