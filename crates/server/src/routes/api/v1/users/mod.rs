//! This file contains all routes dedicated to the users management.

use database::models::users::*;

use crate::prelude::*;
use crate::state::AppState;

/// Builds a router for Kubernetes.
///
/// # Returns
/// An Axum router.
pub fn build() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_me))
        .route("/", get(get_filtered))
        .route("/", post(post_user))
}

/// Handler used to get information about the currently logged user.
#[axum::debug_handler]
async fn get_me(State(state): State<AppState>) -> Res<Json<User>> {
    let user = User::find_by_filters(
        &Filters {
            name: Some("John Doe".to_string()),
            ..Filters::default()
        },
        &state.db,
    )
    .await?
    .first()
    .cloned()
    .ok_or(Error::Database(DatabaseError::NotFound))?;

    Ok(Json(user))
}

/// Handler used to get a list of users that match some filters.
#[axum::debug_handler]
async fn get_filtered(
    Query(filters): Query<Filters>,
    State(state): State<AppState>,
) -> Res<Json<Vec<User>>> {
    let users = User::find_by_filters(&filters, &state.db).await?;

    Ok(Json(users))
}

/// Handler used to create a new user.
async fn post_user(FormOrJson(user): FormOrJson<UserRequest>) -> Json<User> {
    dbg!(&user);

    Json(User {
        name: user.name.clone(),
        email: user.email.clone(),
        ..User::default()
    })
}
