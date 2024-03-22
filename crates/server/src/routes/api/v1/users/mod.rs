//! This file contains all routes dedicated to the users management.

use actions::users::{create_user, set_user_password, update_user};
use database::models::users::*;
use database::traits::sqlx::postgres::crud::*;

use crate::prelude::*;

/// Builds a router for Kubernetes.
///
/// # Returns
/// An Axum router.
pub fn build() -> Router<AppState> {
    Router::new()
        // DELETE
        .route("/:id", delete(delete_by_id))
        // GET
        .route("/", get(get_filtered))
        .route("/me", get(get_me))
        .route("/:id", get(get_by_id))
        // PATCH
        .route("/:id", patch(patch_user))
        .route("/:id/password", patch(patch_user_password))
        // POST
        .route("/", post(post_user))
        // PUT
        .route("/", put(put_user))
}

/// Handler used to delete a user giving its ID.
#[axum::debug_handler]
#[instrument]
async fn delete_by_id(Path(id): Path<Uuid>, State(state): State<AppState>) -> Res<StatusCode> {
    User::delete_by_id(&id, &state.db).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Handler used to get information about the currently logged user.
#[axum::debug_handler]
#[instrument]
async fn get_me(State(state): State<AppState>) -> Res<Json<User>> {
    let user = User::find_by_filters(
        &Filters {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            ..Filters::default()
        },
        &state.db,
    )
    .await?
    .try_into()?;

    Ok(Json(user))
}

/// Handler used to get a list of users that match some filters.
#[axum::debug_handler]
#[instrument]
async fn get_filtered(
    Query(filters): Query<Filters>,
    State(state): State<AppState>,
) -> Res<Json<Vec<User>>> {
    let users = User::find_by_filters(&filters, &state.db).await?;

    Ok(Json(users))
}

/// Handler used to get a specify user by providing its ID.
#[axum::debug_handler]
#[instrument]
async fn get_by_id(Path(id): Path<Uuid>, State(state): State<AppState>) -> Res<Json<User>> {
    let user = User::get(&id, &state.db).await?;

    Ok(Json(user))
}

/// Handler used to create a new user.
#[axum::debug_handler]
#[instrument]
async fn post_user(
    State(state): State<AppState>,
    FormOrJson(request): FormOrJson<UserRequest>,
) -> Res<(StatusCode, Json<User>)> {
    request.validate()?;

    let user = create_user(&request, &state.db).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

/// Handler used to update an existing user by providing its ID.
#[axum::debug_handler]
#[instrument]
async fn patch_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    FormOrJson(request): FormOrJson<UserRequest>,
) -> Res<Json<User>> {
    request.validate()?;

    let user = update_user(&id, &request, &state.db).await?;

    Ok(Json(user))
}

/// Handler used to update an existing user's password by providing its ID.
#[axum::debug_handler]
#[instrument]
async fn patch_user_password(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    FormOrJson(request): FormOrJson<PasswordUpdateRequest>,
) -> Res<StatusCode> {
    request.validate()?;

    set_user_password(&id, &request, &state.db).await?;

    Ok(StatusCode::OK)
}

/// Handler used to upsert a user.
#[axum::debug_handler]
#[instrument]
async fn put_user(
    State(state): State<AppState>,
    FormOrJson(request): FormOrJson<UserRequest>,
) -> Res<(StatusCode, Json<User>)> {
    request.validate()?;

    let (rc, user) = if let Some(id) = request.id {
        let user = update_user(&id, &request, &state.db).await?;
        (StatusCode::OK, user)
    } else {
        let user = create_user(&request, &state.db).await?;
        (StatusCode::CREATED, user)
    };

    Ok((rc, Json(user)))
}
